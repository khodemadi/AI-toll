use anyhow::{bail, Result};
use aitool_core::{ToolInput, ToolRegistry, ToolOutput};
use serde_json::Value;

pub struct Agent {
    registry: ToolRegistry,
}

impl Agent {
    pub fn new(registry: ToolRegistry) -> Self {
        Self { registry }
    }

    pub fn tools(&self) -> Vec<(&str, &str)> {
        self.registry.list()
    }

    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<ToolOutput> {
        let tool = self
            .registry
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("unknown tool: {name}"))?;

        tool.execute(ToolInput::new(arguments)).await
    }

    pub async fn run_once(&self, request: Value) -> Result<ToolOutput> {
        let tool = request
            .get("tool")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("request must contain a string field: tool"))?;

        let arguments = request
            .get("arguments")
            .cloned()
            .unwrap_or(Value::Object(Default::default()));

        self.call_tool(tool, arguments).await
    }
}

