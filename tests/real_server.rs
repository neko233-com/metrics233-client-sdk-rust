#![cfg(feature = "real-server-tests")]

use metrics233_client_sdk_rust::{Client, Error};

#[tokio::test]
async fn actual_server_accepts_native_reports_and_rejects_wrong_token() {
    let root = std::env::var("METRICS233_TEST_ROOT").expect("local contract server required");
    assert!(root.starts_with("http://127.0.0.1:") || root.starts_with("http://[::1]:"));
    let token =
        std::env::var("METRICS233_TEST_INGEST_TOKEN").expect("synthetic test token required");
    let metrics = Client::builder(&root)
        .with_token(token)
        .allow_http_loopback()
        .with_labels([("service", "sdk-contract"), ("implementation", "rust")])
        .build()
        .unwrap();
    metrics.counter("sdk_contract_calls_total", 2.0).unwrap();
    metrics.gauge("sdk_contract_gauge", 233.0).unwrap();
    metrics.shutdown().await.unwrap();
    let wrong = Client::builder(&root)
        .with_token("synthetic-wrong-reporting-token-contract")
        .allow_http_loopback()
        .build()
        .unwrap();
    wrong.gauge("must_not_accept", 1.0).unwrap();
    assert!(matches!(wrong.flush().await, Err(Error::Http(401))));
}
