use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dbdiff")]
#[command(about = "Track database schema changes", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize dbdiff in current directory
    Init {
        /// Database connection string
        #[arg(short, long)]
        connection: Option<String>,
    },

    /// Take a snapshot of the current database schema
    Snapshot {
        /// Optional name for the snapshot
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Compare two snapshots or current schema with latest snapshot
    Diff {
        /// First snapshot ID or "latest"
        #[arg(short, long)]
        from: Option<String>,

        /// Second snapshot ID or "current"
        #[arg(short, long)]
        to: Option<String>,
    },

    /// Show snapshot history
    History {
        /// Number of snapshots to show
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
}
