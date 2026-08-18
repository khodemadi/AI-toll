use anyhow::{Context, Result};
use async_trait::async_trait;
use aitool_core::{Tool, ToolInput, ToolOutput};
use serde_json::json;
use tokio::process::Command;

pub struct ShellTool;

impl ShellTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ShellTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for ShellTool {
    fn name(&self) -> &str {
        "shell"
    }

    fn description(&self) -> &str {
        "Run a shell command. Input: {\"command\":\"...\"}"
    }

    async fn execute(&self, input: ToolInput) -> Result<ToolOutput> {
        let command = input
            .arguments
            .get("command")
            .and_then(|v| v.as_str())
            .context("missing string argument: command")?;

        #[cfg(target_os = "windows")]
        let output = Command::new("cmd")
            .args(["/C", command])
            .output()
            .await
            .context("failed to execute command")?;

        #[cfg(not(target_os = "windows"))]
        let output = Command::new("sh")
            .args(["-c", command])
            .output()
            .await
            .context("failed to execute command")?;

        Ok(ToolOutput::success(json!({
            "status": output.status.code(),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr)
        })))
    }
}

