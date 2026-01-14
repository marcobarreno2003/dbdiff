use anyhow::Result;

pub async fn execute(from: Option<String>, to: Option<String>) -> Result<()> {
    let from_ref = from.unwrap_or_else(|| "latest".to_string());
    let to_ref = to.unwrap_or_else(|| "current".to_string());

    println!("Comparing {} -> {}", from_ref, to_ref);

    // TODO: Load snapshots
    // TODO: Run diff engine
    // TODO: Display results

    Ok(())
}
