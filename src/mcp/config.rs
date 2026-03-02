use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level MCP configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct McpConfig {
    /// Whether MCP client support is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Named MCP server configurations.
    #[serde(default)]
    pub servers: HashMap<String, McpServerConfig>,
}

/// Configuration for a single MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Whether this server is enabled (default: true).
    /// Set to false to skip connecting on startup.
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Transport type: "stdio" (default) or "sse".
    #[serde(default = "default_transport")]
    pub transport: String,
    /// Command to spawn (stdio transport).
    #[serde(default)]
    pub command: Option<String>,
    /// Arguments for the command (stdio transport).
    #[serde(default)]
    pub args: Vec<String>,
    /// Environment variables for the subprocess.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// URL for SSE transport.
    #[serde(default)]
    pub url: Option<String>,
    /// HTTP headers for SSE transport (e.g. Authorization).
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// Timeout in seconds for tool calls.
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
    /// Auto-restart subprocess on crash (stdio only).
    #[serde(default = "default_auto_restart")]
    pub auto_restart: bool,
}

fn default_enabled() -> bool {
    true
}

fn default_transport() -> String {
    "stdio".into()
}

fn default_timeout_secs() -> u64 {
    30
}

fn default_auto_restart() -> bool {
    true
}

impl Default for McpServerConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            transport: default_transport(),
            command: None,
            args: Vec::new(),
            env: HashMap::new(),
            url: None,
            headers: HashMap::new(),
            timeout_secs: default_timeout_secs(),
            auto_restart: default_auto_restart(),
        }
    }
}
