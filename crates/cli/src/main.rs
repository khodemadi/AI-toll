use anyhow::Result;
use aitool_agent::Agent;
use aitool_tools::default_registry;
use clap::{Parser, Subcommand};
use serde_json::json;

#[derive(Parser, Debug)]
#[command(name = "aitool", version, about = "AI agent toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List registered tools
    Tools,

    /// Read a text file
    FileRead {
        /// Path to the file
        path: String,
    },

    /// Execute a shell command
    Shell {
        /// Command to execute
        command: String,
    },

    /// Execute a raw tool call
    Call {
        /// Tool name
        tool: String,

        /// JSON arguments
        #[arg(default_value = "{}")]
        arguments: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let agent = Agent::new(default_registry());

    match cli.command {
        Commands::Tools => {
            for (name, description) in agent.tools() {
                println!("{name}\n  {description}\n");
            }
        }

        Commands::FileRead { path } => {
            let result = agent
                .call_tool("file_read", json!({ "path": path }))
                .await?;

            println!("{}", serde_json::to_string_pretty(&result)?);
        }

        Commands::Shell { command } => {
            let result = agent
                .call_tool("shell", json!({ "command": command }))
                .await?;

            println!("{}", serde_json::to_string_pretty(&result)?);
        }

        Commands::Call { tool, arguments } => {
            let arguments: serde_json::Value = serde_json::from_str(&arguments)?;
            let result = agent.call_tool(&tool, arguments).await?;

            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}

