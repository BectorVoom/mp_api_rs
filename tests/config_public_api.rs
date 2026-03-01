use std::time::Duration;

use mp_api_rs::{ConfigBuilder, RetryConfig};

#[test]
fn public_builder_api_constructs_config() {
    let retry = RetryConfig::new(4, Duration::from_millis(500), Duration::from_secs(3))
        .expect("valid retry config should build");
    let config = ConfigBuilder::new()
        .api_key("integration-key")
        .base_url("https://example.com/api")
        .timeout(Duration::from_secs(15))
        .concurrency(4)
        .qps_limit(10)
        .user_agent("integration-test/1.0")
        .retry(retry.clone())
        .build()
        .expect("public API should build config");

    assert_eq!(config.api_key(), "integration-key");
    assert_eq!(config.base_url().as_str(), "https://example.com/api/");
    assert_eq!(config.timeout(), Some(Duration::from_secs(15)));
    assert_eq!(config.concurrency(), 4);
    assert_eq!(config.qps_limit(), 10);
    assert_eq!(config.user_agent(), "integration-test/1.0");
    assert_eq!(config.retry(), &retry);
}

#[test]
fn public_retry_config_rejects_invalid_backoff_ranges() {
    let error = RetryConfig::new(1, Duration::from_secs(5), Duration::from_secs(1))
        .expect_err("inverted backoff range should fail");

    assert!(matches!(
        error,
        mp_api_rs::MpApiError::InvalidConfiguration { field: "retry", .. }
    ));
}
