use anyhow::Result;

pub async fn execute(limit: u32) -> Result<()> {
    println!("Showing last {} snapshots", limit);

    // TODO: Query SQLite for snapshots
    // TODO: Display formatted list

    Ok(())
}
