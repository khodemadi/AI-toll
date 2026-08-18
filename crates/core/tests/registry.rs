use aitool_core::{Tool, ToolInput, ToolOutput, ToolRegistry};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;

struct TestTool;

#[async_trait]
impl Tool for TestTool {
    fn name(&self) -> &str { "test" }
    fn description(&self) -> &str { "test tool" }

    async fn execute(&self, input: ToolInput) -> Result<ToolOutput> {
        Ok(ToolOutput::success(input.arguments))
    }
}

#[tokio::test]
async fn registry_can_register_and_execute() {
    let mut registry = ToolRegistry::new();
    registry.register(TestTool);

    let tool = registry.get("test").unwrap();
    let output = tool.execute(ToolInput::new(json!({"ok": true}))).await.unwrap();

    assert!(output.success);
    assert_eq!(output.output["ok"], true);
}

