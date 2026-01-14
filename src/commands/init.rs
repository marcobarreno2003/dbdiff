use anyhow::Result;

pub async fn execute(connection: Option<String>) -> Result<()> {
    println!("Initializing dbdiff...");

    if let Some(conn) = connection {
        println!("Connection string: {}", conn);
    }

    // TODO: Create .dbdiff directory
    // TODO: Create config file
    // TODO: Initialize SQLite database

    Ok(())
}
