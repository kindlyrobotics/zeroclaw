//! MCP Orchestrator - Execute multiple MCP actions and aggregate results
//!
//! This module provides a high-level interface for:
//! - Executing multiple MCP tool calls in sequence or parallel
//! - Aggregating results with success/failure tracking
//! - Retry logic and error handling
//! - Result summarization and reporting

use super::client::McpClient;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Orchestrator for executing multiple MCP actions and aggregating results
pub struct McpOrchestrator {
    client: Arc<McpClient>,
    config: OrchestratorConfig,
}

/// Configuration for the orchestrator
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    /// Maximum number of retry attempts for failed actions
    pub max_retries: usize,
    /// Delay between retry attempts
    pub retry_delay: Duration,
    /// Whether to stop on first failure
    pub fail_fast: bool,
    /// Execute actions in parallel (when dependencies allow)
    pub parallel: bool,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            max_retries: 2,
            retry_delay: Duration::from_secs(1),
            fail_fast: false,
            parallel: false,
        }
    }
}

impl McpOrchestrator {
    /// Create a new orchestrator for the given MCP client
    pub fn new(client: Arc<McpClient>) -> Self {
        Self {
            client,
            config: OrchestratorConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(client: Arc<McpClient>, config: OrchestratorConfig) -> Self {
        Self { client, config }
    }

    /// Execute a single action with retry logic
    pub async fn execute_action(&self, action: &Action) -> ActionResult {
        let start = Instant::now();
        let mut attempts = 0;
        let mut last_error = None;

        while attempts <= self.config.max_retries {
            attempts += 1;

            match self.try_execute(action).await {
                Ok(output) => {
                    return ActionResult {
                        action_name: action.name.clone(),
                        success: true,
                        output,
                        error: None,
                        attempts,
                        duration: start.elapsed(),
                    };
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    
                    if attempts <= self.config.max_retries {
                        tracing::warn!(
                            action = %action.name,
                            attempt = attempts,
                            max_retries = self.config.max_retries,
                            error = %e,
                            "Action failed, retrying..."
                        );
                        tokio::time::sleep(self.config.retry_delay).await;
                    }
                }
            }
        }

        ActionResult {
            action_name: action.name.clone(),
            success: false,
            output: String::new(),
            error: last_error,
            attempts,
            duration: start.elapsed(),
        }
    }

    /// Execute multiple actions and aggregate results
    pub async fn execute_batch(&self, actions: Vec<Action>) -> BatchResult {
        let start = Instant::now();
        let total = actions.len();
        let mut results = Vec::with_capacity(total);

        tracing::info!(
            actions = total,
            parallel = self.config.parallel,
            "Executing action batch"
        );

        if self.config.parallel {
            // Execute in parallel using join_all
            let futures: Vec<_> = actions
                .iter()
                .map(|action| self.execute_action(action))
                .collect();

            results = futures::future::join_all(futures).await;
        } else {
            // Execute sequentially
            for action in &actions {
                let result = self.execute_action(action).await;
                
                if self.config.fail_fast && !result.success {
                    tracing::warn!(
                        action = %action.name,
                        "Action failed in fail-fast mode, stopping batch"
                    );
                    results.push(result);
                    break;
                }
                
                results.push(result);
            }
        }

        let successful = results.iter().filter(|r| r.success).count();
        let failed = results.iter().filter(|r| !r.success).count();

        BatchResult {
            total,
            successful,
            failed,
            results,
            duration: start.elapsed(),
        }
    }

    /// Execute actions and return a summary
    pub async fn execute_and_summarize(&self, actions: Vec<Action>) -> Summary {
        let batch_result = self.execute_batch(actions).await;
        Summary::from_batch_result(batch_result)
    }

    /// Internal: Try to execute an action once (no retry)
    async fn try_execute(&self, action: &Action) -> Result<String> {
        let result = self
            .client
            .call_tool(&action.name, action.args.clone())
            .await
            .with_context(|| format!("Failed to call tool '{}'", action.name))?;

        if result.is_error {
            let error_text = result
                .content
                .iter()
                .filter_map(|c| c.text.as_deref())
                .collect::<Vec<_>>()
                .join("\n");
            anyhow::bail!("Tool returned error: {}", error_text);
        }

        let output = result
            .content
            .iter()
            .filter_map(|c| c.text.as_deref())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(output)
    }
}

/// An action to execute on an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Name of the MCP tool to call
    pub name: String,
    /// Arguments to pass to the tool
    pub args: Value,
    /// Optional description for logging/reporting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Action {
    /// Create a new action
    pub fn new(name: impl Into<String>, args: Value) -> Self {
        Self {
            name: name.into(),
            args,
            description: None,
        }
    }

    /// Add a description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Result of executing a single action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Name of the action that was executed
    pub action_name: String,
    /// Whether the action succeeded
    pub success: bool,
    /// Output from the action (if successful)
    pub output: String,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Number of attempts made
    pub attempts: usize,
    /// Time taken to execute (including retries)
    #[serde(serialize_with = "serialize_duration")]
    pub duration: Duration,
}

/// Result of executing a batch of actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    /// Total number of actions
    pub total: usize,
    /// Number of successful actions
    pub successful: usize,
    /// Number of failed actions
    pub failed: usize,
    /// Individual action results
    pub results: Vec<ActionResult>,
    /// Total time taken for the batch
    #[serde(serialize_with = "serialize_duration")]
    pub duration: Duration,
}

impl BatchResult {
    /// Calculate success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.successful as f64 / self.total as f64) * 100.0
        }
    }

    /// Get all successful results
    pub fn successful_results(&self) -> Vec<&ActionResult> {
        self.results.iter().filter(|r| r.success).collect()
    }

    /// Get all failed results
    pub fn failed_results(&self) -> Vec<&ActionResult> {
        self.results.iter().filter(|r| !r.success).collect()
    }
}

/// High-level summary of execution results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    /// Total actions executed
    pub total: usize,
    /// Number of successful actions
    pub successful: usize,
    /// Number of failed actions
    pub failed: usize,
    /// Success rate percentage
    pub success_rate: f64,
    /// Total execution time
    #[serde(serialize_with = "serialize_duration")]
    pub total_duration: Duration,
    /// Average time per action
    #[serde(serialize_with = "serialize_duration")]
    pub avg_duration: Duration,
    /// Concatenated output from all successful actions
    pub combined_output: String,
    /// List of errors from failed actions
    pub errors: Vec<String>,
}

impl Summary {
    /// Create a summary from a batch result
    pub fn from_batch_result(batch: BatchResult) -> Self {
        let combined_output = batch
            .successful_results()
            .iter()
            .map(|r| r.output.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");

        let errors = batch
            .failed_results()
            .iter()
            .filter_map(|r| r.error.clone())
            .collect();

        let avg_duration = if batch.total > 0 {
            batch.duration / batch.total as u32
        } else {
            Duration::from_secs(0)
        };

        Self {
            total: batch.total,
            successful: batch.successful,
            failed: batch.failed,
            success_rate: batch.success_rate(),
            total_duration: batch.duration,
            avg_duration,
            combined_output,
            errors,
        }
    }

    /// Print a formatted summary to stdout
    pub fn print(&self) {
        println!("\n📊 Execution Summary");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Total Actions:    {}", self.total);
        println!("✅ Successful:    {} ({:.1}%)", self.successful, self.success_rate);
        println!("❌ Failed:        {}", self.failed);
        println!("⏱️  Total Time:    {:?}", self.total_duration);
        println!("⌀  Avg Time:      {:?}", self.avg_duration);
        
        if !self.errors.is_empty() {
            println!("\n⚠️  Errors:");
            for (i, error) in self.errors.iter().enumerate() {
                println!("  {}. {}", i + 1, error);
            }
        }
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

/// Custom serializer for Duration
fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{:.3}s", duration.as_secs_f64()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_builder_works() {
        let action = Action::new("test_tool", serde_json::json!({"key": "value"}))
            .with_description("Test action");

        assert_eq!(action.name, "test_tool");
        assert_eq!(action.description, Some("Test action".to_string()));
    }

    #[test]
    fn batch_result_calculates_success_rate() {
        let batch = BatchResult {
            total: 10,
            successful: 7,
            failed: 3,
            results: vec![],
            duration: Duration::from_secs(5),
        };

        assert_eq!(batch.success_rate(), 70.0);
    }

    #[test]
    fn batch_result_handles_zero_total() {
        let batch = BatchResult {
            total: 0,
            successful: 0,
            failed: 0,
            results: vec![],
            duration: Duration::from_secs(0),
        };

        assert_eq!(batch.success_rate(), 0.0);
    }
}
