use anyhow::Result;

pub async fn execute(name: Option<String>) -> Result<()> {
    let snapshot_name = name.unwrap_or_else(|| {
        chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string()
    });

    println!("Taking snapshot: {}", snapshot_name);

    // TODO: Connect to database
    // TODO: Extract schema
    // TODO: Save snapshot to SQLite

    Ok(())
}
