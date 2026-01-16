mod cli;
mod commands;
mod config;
mod schema;
mod storage;
mod diff;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { connection } => {
            commands::init::execute(connection).await?;
        }
        Commands::Snapshot { name } => {
            commands::snapshot::execute(name).await?;
        }
        Commands::Diff { from, to } => {
            commands::diff::execute(from, to).await?;
        }
        Commands::History { limit } => {
            commands::history::execute(limit).await?;
        }
    }

    Ok(())
}
