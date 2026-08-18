use anyhow::{Context, Result};
use async_trait::async_trait;
use aitool_core::{Tool, ToolInput, ToolOutput};
use serde_json::json;
use tokio::fs;

pub struct FileReadTool;

impl FileReadTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileReadTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "file_read"
    }

    fn description(&self) -> &str {
        "Read a UTF-8 text file. Input: {\"path\":\"...\"}"
    }

    async fn execute(&self, input: ToolInput) -> Result<ToolOutput> {
        let path = input
            .arguments
            .get("path")
            .and_then(|v| v.as_str())
            .context("missing string argument: path")?;

        let content = fs::read_to_string(path)
            .await
            .with_context(|| format!("failed to read file: {path}"))?;

        Ok(ToolOutput::success(json!({
            "path": path,
            "content": content
        })))
    }
}

