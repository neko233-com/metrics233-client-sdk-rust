use metrics233_client_sdk_rust::{Client, Identity, Limits};

#[test]
fn entrypoint_shares_explicit_lifecycle_and_bounds() {
    let client = Client::builder("https://monitoring.example.com")
        .with_token("synthetic-reporting-token-contract-32-chars")
        .with_identity(
            Identity::new("1")
                .with_service("fixture")
                .with_role("logic"),
        )
        .with_limits(Limits {
            max_series: 1,
            max_pending: 1,
            batch_size: 1,
        })
        .build()
        .unwrap();
    client.gauge("online", 0.0).unwrap();
    assert!(client.gauge("different", 1.0).is_err());
    assert!(client.counter("online", 1.0).is_err());
}

#[tokio::test]
async fn empty_shutdown_is_explicit_and_idempotent() {
    let client = Client::builder("https://monitoring.example.com")
        .with_token("synthetic-reporting-token-contract-32-chars")
        .build()
        .unwrap();
    client.shutdown().await.unwrap();
    client.shutdown().await.unwrap();
    assert!(client.gauge("after_shutdown", 1.0).is_err());
}
