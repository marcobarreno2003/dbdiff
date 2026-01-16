use anyhow::{Context, Result};
use colored::*;

use crate::config::Config;

pub async fn execute(connection: Option<String>) -> Result<()> {
    if Config::is_initialized() {
        println!("{}", "dbdiff is already initialized in this directory.".yellow());
        return Ok(());
    }

    let connection_string = match connection {
        Some(conn) => conn,
        None => {
            anyhow::bail!(
                "Connection string required. Use: dbdiff init -c <connection_string>\n\
                 Example: dbdiff init -c \"postgres://user:pass@localhost/mydb\""
            );
        }
    };

    // Test connection before saving
    println!("Testing connection...");
    sqlx::postgres::PgPool::connect(&connection_string)
        .await
        .context("Failed to connect to database. Check your connection string.")?;

    println!("{}", "Connection successful!".green());

    // Initialize config
    Config::init(connection_string)?;

    println!("{}", "Initialized dbdiff in .dbdiff/".green());
    println!("Next steps:");
    println!("  dbdiff snapshot    - Take a snapshot of current schema");
    println!("  dbdiff history     - View snapshot history");
    println!("  dbdiff diff        - Compare snapshots");

    Ok(())
}
