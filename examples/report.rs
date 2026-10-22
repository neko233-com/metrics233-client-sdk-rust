use metrics233_client_sdk_rust::{Client, Identity};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metrics = Client::builder(std::env::var("METRICS233_URL")?)
        .with_token(std::env::var("METRICSPANEL_INGEST_TOKEN")?)
        .with_identity(
            Identity::new("1")
                .with_service("game-api")
                .with_role("logic")
                .with_environment("test")
                .with_instance("node-1"),
        )
        .with_labels([("implementation", "rust")])
        .build()?;
    metrics
        .batch()
        .counter("game_requests_total", 1.0)
        .gauge("game_players_online", 42.0)
        .commit()?;
    metrics.shutdown().await?;
    Ok(())
}
