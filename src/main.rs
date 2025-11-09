use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber;

use aishell::cli::Repl;
use aishell::mcp::MCPServer;

#[derive(Parser)]
#[command(name = "aishell")]
#[command(about = "AI-powered shell automation - A generic alternative to Claude Code")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive AI shell
    Shell {
        /// LLM provider (openai, anthropic, ollama)
        #[arg(short, long, default_value = "openai")]
        provider: String,

        /// Model name
        #[arg(short, long)]
        model: Option<String>,
    },

    /// Execute a single command via AI
    Exec {
        /// Command prompt
        prompt: String,

        /// LLM provider
        #[arg(short = 'p', long, default_value = "openai")]
        provider: String,
    },

    /// Start MCP server (for Claude Desktop integration)
    Server,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Shell { provider, model } => {
            let mut repl = Repl::new(&provider, model.as_deref()).await?;
            repl.run().await?;
        }

        Commands::Exec { prompt, provider } => {
            let mut repl = Repl::new(&provider, None).await?;
            repl.execute_once(&prompt).await?;
        }

        Commands::Server => {
            let server = MCPServer::new()?;
            server.run().await?;
        }
    }

    Ok(())
}
