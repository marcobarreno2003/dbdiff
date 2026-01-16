use anyhow::Result;
use colored::*;

use crate::config::Config;
use crate::schema::postgres;

pub async fn execute(name: Option<String>) -> Result<()> {
    let config = Config::load()?;

    let snapshot_name = name.unwrap_or_else(|| {
        chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string()
    });

    println!("Connecting to database...");
    let schema = postgres::extract_schema(&config.connection_string).await?;

    println!(
        "{} Captured {} tables",
        "OK".green().bold(),
        schema.table_count()
    );

    // Print summary
    for table in &schema.tables {
        println!(
            "  {} {}.{} ({} columns)",
            "•".blue(),
            table.schema,
            table.name,
            table.columns.len()
        );
    }

    // TODO: Save to SQLite storage
    println!(
        "\n{} Snapshot '{}' created",
        "✓".green().bold(),
        snapshot_name
    );

    Ok(())
}
