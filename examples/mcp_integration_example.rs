/// Example: Connect to MCP server, execute multiple tools, and aggregate results
///
/// This demonstrates:
/// 1. Connecting to an MCP server (stdio or SSE transport)
/// 2. Discovering available tools
/// 3. Executing multiple tool calls
/// 4. Aggregating and summarizing results
///
/// Run with:
/// ```bash
/// cargo run --example mcp_integration_example
/// ```

use anyhow::Result;
use serde_json::{json, Value};
use std::sync::Arc;
use zeroclaw::mcp::{client::McpClient, config::McpServerConfig, transport::StdioTransport};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🦀 ZeroClaw MCP Integration Example\n");

    // Example 1: Connect to filesystem MCP server
    let result = connect_and_aggregate_results().await?;
    
    println!("\n📊 Aggregated Results Summary:");
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}

/// Connect to an MCP server and aggregate results from multiple tool calls
async fn connect_and_aggregate_results() -> Result<Value> {
    // Step 1: Configure the MCP server connection
    let server_config = McpServerConfig {
        command: Some("npx".to_string()),
        args: vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-filesystem".to_string(),
            "/tmp".to_string(),
        ],
        env: std::collections::HashMap::new(),
        transport: "stdio".to_string(),
        url: None,
        timeout_secs: 30,
        auto_restart: true,
    };

    println!("🔌 Connecting to MCP server...");
    
    // Step 2: Create transport and client
    let transport = Box::new(StdioTransport::spawn(
        server_config.command.as_ref().unwrap(),
        &server_config.args,
        &server_config.env,
        server_config.auto_restart,
    )?);

    let mut client = McpClient::new(
        "filesystem".to_string(),
        transport,
        server_config.timeout_secs,
    );

    // Step 3: Initialize connection
    let init_result = client.initialize().await?;
    println!("✅ Connected to: {}", init_result.server_info
        .as_ref()
        .map(|s| s.name.as_str())
        .unwrap_or("unknown"));

    // Step 4: Discover available tools
    let tools = client.list_tools().await?;
    println!("🔧 Available tools: {}", tools.len());
    for tool in &tools {
        println!("   - {} {}", 
            tool.name,
            tool.description.as_deref().unwrap_or("")
        );
    }

    // Step 5: Execute multiple actions and aggregate results
    let mut results = Vec::new();
    let mut summary = AggregatedResults::new();

    // Example actions to perform
    let actions = vec![
        ToolAction {
            name: "list_directory",
            args: json!({ "path": "/tmp" }),
            description: "List /tmp directory",
        },
        ToolAction {
            name: "get_file_info",
            args: json!({ "path": "/tmp" }),
            description: "Get /tmp info",
        },
    ];

    println!("\n🚀 Executing {} actions...\n", actions.len());

    for action in actions {
        println!("▶ {}: {}", action.name, action.description);
        
        match client.call_tool(&action.name, action.args.clone()).await {
            Ok(result) => {
                let output = result.content
                    .iter()
                    .filter_map(|c| c.text.as_deref())
                    .collect::<Vec<_>>()
                    .join("\n");

                let action_result = ActionResult {
                    action: action.name.clone(),
                    success: !result.is_error,
                    output: output.clone(),
                    error: if result.is_error { Some(output) } else { None },
                };

                if action_result.success {
                    summary.successful += 1;
                    println!("  ✅ Success");
                } else {
                    summary.failed += 1;
                    println!("  ❌ Failed: {}", action_result.error.as_deref().unwrap_or("unknown"));
                }

                results.push(action_result);
            }
            Err(e) => {
                summary.failed += 1;
                println!("  ❌ Error: {}", e);
                
                results.push(ActionResult {
                    action: action.name.clone(),
                    success: false,
                    output: String::new(),
                    error: Some(e.to_string()),
                });
            }
        }
    }

    // Step 6: Graceful shutdown
    client.shutdown().await?;
    println!("\n🔌 Disconnected from MCP server");

    // Step 7: Return aggregated summary
    summary.total = results.len();
    summary.results = results;

    Ok(serde_json::to_value(summary)?)
}

/// Represents a single tool action to execute
#[derive(Debug, Clone)]
struct ToolAction {
    name: String,
    args: Value,
    description: String,
}

/// Result of a single action
#[derive(Debug, Clone, serde::Serialize)]
struct ActionResult {
    action: String,
    success: bool,
    output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Aggregated results from multiple actions
#[derive(Debug, serde::Serialize)]
struct AggregatedResults {
    total: usize,
    successful: usize,
    failed: usize,
    results: Vec<ActionResult>,
}

impl AggregatedResults {
    fn new() -> Self {
        Self {
            total: 0,
            successful: 0,
            failed: 0,
            results: Vec::new(),
        }
    }
}
