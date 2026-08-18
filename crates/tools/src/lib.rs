pub mod filesystem;
pub mod shell;

pub use filesystem::FileReadTool;
pub use shell::ShellTool;

use aitool_core::ToolRegistry;

pub fn default_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();

    registry.register(FileReadTool::new());
    registry.register(ShellTool::new());

    registry
}

