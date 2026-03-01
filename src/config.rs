use std::fmt;
use std::time::Duration;

use url::{Host, Url};

use crate::error::MpApiError;
use crate::middleware::retry::RetryConfig;

const DEFAULT_BASE_URL: &str = "https://api.materialsproject.org/";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_CONCURRENCY: usize = 16;
const DEFAULT_QPS_LIMIT: u32 = 25;

#[derive(Clone)]
pub struct Config {
    api_key: String,
    base_url: Url,
    timeout: Option<Duration>,
    concurrency: usize,
    qps_limit: u32,
    user_agent: String,
    allow_insecure_http: bool,
    retry: RetryConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, MpApiError> {
        load_dotenv_if_present(dotenvy::dotenv)?;
        ConfigBuilder::new().build_with_env(|key| std::env::var(key).ok())
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }

    pub fn qps_limit(&self) -> u32 {
        self.qps_limit
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    pub fn allow_insecure_http(&self) -> bool {
        self.allow_insecure_http
    }

    pub fn retry(&self) -> &RetryConfig {
        &self.retry
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let redacted_base_url = redact_url_for_debug(&self.base_url);

        f.debug_struct("Config")
            .field("api_key", &"<redacted>")
            .field("base_url", &redacted_base_url)
            .field("timeout", &self.timeout)
            .field("concurrency", &self.concurrency)
            .field("qps_limit", &self.qps_limit)
            .field("user_agent", &self.user_agent)
            .field("allow_insecure_http", &self.allow_insecure_http)
            .field("retry", &self.retry)
            .finish()
    }
}

#[derive(Clone, Default)]
pub struct ConfigBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Option<Duration>>,
    concurrency: Option<usize>,
    qps_limit: Option<u32>,
    user_agent: Option<String>,
    allow_insecure_http: Option<bool>,
    retry: Option<RetryConfig>,
}

impl fmt::Debug for ConfigBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let redacted_api_key = self.api_key.as_ref().map(|_| "<redacted>");
        let redacted_base_url = self.base_url.as_deref().map(redact_url_input_for_debug);

        f.debug_struct("ConfigBuilder")
            .field("api_key", &redacted_api_key)
            .field("base_url", &redacted_base_url)
            .field("timeout", &self.timeout)
            .field("concurrency", &self.concurrency)
            .field("qps_limit", &self.qps_limit)
            .field("user_agent", &self.user_agent)
            .field("allow_insecure_http", &self.allow_insecure_http)
            .field("retry", &self.retry)
            .finish()
    }
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn base_url(mut self, base_url: impl AsRef<str>) -> Self {
        self.base_url = Some(base_url.as_ref().to_string());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(Some(timeout));
        self
    }

    pub fn no_timeout(mut self) -> Self {
        self.timeout = Some(None);
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn qps_limit(mut self, qps_limit: u32) -> Self {
        self.qps_limit = Some(qps_limit);
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn allow_insecure_http(mut self, allow_insecure_http: bool) -> Self {
        self.allow_insecure_http = Some(allow_insecure_http);
        self
    }

    pub fn retry(mut self, retry: RetryConfig) -> Self {
        self.retry = Some(retry);
        self
    }

    pub fn build(self) -> Result<Config, MpApiError> {
        self.build_with_env(|key| std::env::var(key).ok())
    }

    fn build_with_env<F>(self, mut env_lookup: F) -> Result<Config, MpApiError>
    where
        F: FnMut(&str) -> Option<String>,
    {
        let Self {
            api_key,
            base_url,
            timeout,
            concurrency,
            qps_limit,
            user_agent,
            allow_insecure_http,
            retry,
        } = self;

        let allow_insecure_http = allow_insecure_http.unwrap_or(false);

        Ok(Config {
            api_key: resolve_api_key(api_key, &mut env_lookup)?,
            base_url: resolve_base_url(base_url, allow_insecure_http)?,
            timeout: timeout.unwrap_or(Some(Duration::from_secs(DEFAULT_TIMEOUT_SECS))),
            concurrency: validate_concurrency(concurrency.unwrap_or(DEFAULT_CONCURRENCY))?,
            qps_limit: validate_qps_limit(qps_limit.unwrap_or(DEFAULT_QPS_LIMIT))?,
            user_agent: resolve_user_agent(user_agent)?,
            allow_insecure_http,
            retry: retry.unwrap_or_default(),
        })
    }
}

fn load_dotenv_if_present<F>(loader: F) -> Result<(), MpApiError>
where
    F: FnOnce() -> Result<std::path::PathBuf, dotenvy::Error>,
{
    match loader() {
        Ok(_) => Ok(()),
        Err(error) if error.not_found() => Ok(()),
        Err(error) => Err(MpApiError::InvalidConfiguration {
            field: ".env",
            message: format!("failed to load .env: {error}"),
        }),
    }
}

fn resolve_api_key<F>(
    builder_api_key: Option<String>,
    env_lookup: &mut F,
) -> Result<String, MpApiError>
where
    F: FnMut(&str) -> Option<String>,
{
    if let Some(api_key) = builder_api_key {
        return validate_non_empty("api_key", api_key);
    }

    for env_key in ["MP_API_KEY", "PMG_MAPI_KEY"] {
        if let Some(api_key) = env_lookup(env_key)
            && let Ok(api_key) = validate_non_empty("api_key", api_key)
        {
            return Ok(api_key);
        }
    }

    Err(MpApiError::MissingApiKey)
}

fn resolve_base_url(
    configured_base_url: Option<String>,
    allow_insecure_http: bool,
) -> Result<Url, MpApiError> {
    let input = configured_base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
    let mut parsed = Url::parse(&input).map_err(|_| MpApiError::InvalidBaseUrl)?;

    if url_has_userinfo(&parsed) {
        return Err(MpApiError::InvalidBaseUrl);
    }

    match parsed.scheme() {
        "https" => {
            normalize_base_url_path(&mut parsed);
            Ok(parsed)
        }
        "http" if allow_insecure_http && is_local_http_host(&parsed) => {
            normalize_base_url_path(&mut parsed);
            Ok(parsed)
        }
        "http" => Err(MpApiError::InsecureBaseUrlNotAllowed),
        _ => Err(MpApiError::InvalidBaseUrl),
    }
}

fn url_has_userinfo(base_url: &Url) -> bool {
    !base_url.username().is_empty() || base_url.password().is_some()
}

fn is_local_http_host(base_url: &Url) -> bool {
    match base_url.host() {
        Some(Host::Domain(domain)) => domain.eq_ignore_ascii_case("localhost"),
        Some(Host::Ipv4(address)) => address.is_loopback(),
        Some(Host::Ipv6(address)) => address.is_loopback(),
        None => false,
    }
}

fn normalize_base_url_path(base_url: &mut Url) {
    if !base_url.path().ends_with('/') {
        let normalized_path = format!("{}/", base_url.path());
        base_url.set_path(&normalized_path);
    }
}

fn redact_url_for_debug(base_url: &Url) -> String {
    let mut redacted = base_url.clone();

    if !redacted.username().is_empty() {
        let _ = redacted.set_username("");
    }

    if redacted.password().is_some() {
        let _ = redacted.set_password(None);
    }

    redacted.set_query(None);
    redacted.set_fragment(None);

    redacted.to_string()
}

fn redact_url_input_for_debug(base_url: &str) -> String {
    Url::parse(base_url)
        .map(|parsed| redact_url_for_debug(&parsed))
        .unwrap_or_else(|_| "<redacted-invalid-url>".to_string())
}

fn resolve_user_agent(user_agent: Option<String>) -> Result<String, MpApiError> {
    let user_agent = user_agent.unwrap_or_else(default_user_agent);
    validate_non_empty("user_agent", user_agent)
}

fn default_user_agent() -> String {
    format!("mp-api-rs/{}", env!("CARGO_PKG_VERSION"))
}

fn validate_concurrency(concurrency: usize) -> Result<usize, MpApiError> {
    if concurrency == 0 {
        return Err(MpApiError::InvalidConfiguration {
            field: "concurrency",
            message: "must be at least 1".to_string(),
        });
    }

    Ok(concurrency)
}

fn validate_qps_limit(qps_limit: u32) -> Result<u32, MpApiError> {
    if qps_limit == 0 {
        return Err(MpApiError::InvalidConfiguration {
            field: "qps_limit",
            message: "must be at least 1".to_string(),
        });
    }

    Ok(qps_limit)
}

fn validate_non_empty(field: &'static str, value: String) -> Result<String, MpApiError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(MpApiError::InvalidConfiguration {
            field,
            message: "must not be empty".to_string(),
        });
    }

    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::time::Duration;

    use super::{
        Config, ConfigBuilder, DEFAULT_BASE_URL, DEFAULT_CONCURRENCY, DEFAULT_QPS_LIMIT,
        DEFAULT_TIMEOUT_SECS, load_dotenv_if_present,
    };
    use crate::error::MpApiError;
    use crate::middleware::retry::RetryConfig;

    fn build_with_explicit_api_key() -> Result<Config, MpApiError> {
        ConfigBuilder::new()
            .api_key("test-api-key")
            .build_with_env(|_| None)
    }

    fn build_with_fake_env(
        builder: ConfigBuilder,
        env_values: &[(&str, &str)],
    ) -> Result<Config, MpApiError> {
        builder.build_with_env(|key| {
            env_values
                .iter()
                .find_map(|(env_key, value)| (*env_key == key).then(|| (*value).to_string()))
        })
    }

    #[test]
    fn builder_api_key_takes_precedence_over_environment() {
        let config = build_with_fake_env(
            ConfigBuilder::new().api_key("builder-key"),
            &[
                ("MP_API_KEY", "env-primary"),
                ("PMG_MAPI_KEY", "env-secondary"),
            ],
        )
        .expect("builder api key should win");

        assert_eq!(config.api_key(), "builder-key");
    }

    #[test]
    fn mp_api_key_is_used_when_builder_api_key_is_absent() {
        let config = build_with_fake_env(ConfigBuilder::new(), &[("MP_API_KEY", "env-primary")])
            .expect("MP_API_KEY should be used");

        assert_eq!(config.api_key(), "env-primary");
    }

    #[test]
    fn pmg_mapi_key_is_used_when_primary_sources_are_absent() {
        let config =
            build_with_fake_env(ConfigBuilder::new(), &[("PMG_MAPI_KEY", "env-secondary")])
                .expect("PMG_MAPI_KEY should be used");

        assert_eq!(config.api_key(), "env-secondary");
    }

    #[test]
    fn missing_api_key_returns_missing_api_key_error() {
        let error =
            build_with_fake_env(ConfigBuilder::new(), &[]).expect_err("missing key should fail");

        assert_eq!(error, MpApiError::MissingApiKey);
    }

    #[test]
    fn default_values_match_spec() {
        let config = build_with_explicit_api_key().expect("explicit key should build");

        assert_eq!(config.base_url().as_str(), DEFAULT_BASE_URL);
        assert_eq!(
            config.timeout(),
            Some(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
        );
        assert_eq!(config.concurrency(), DEFAULT_CONCURRENCY);
        assert_eq!(config.qps_limit(), DEFAULT_QPS_LIMIT);
        assert_eq!(
            config.user_agent(),
            format!("mp-api-rs/{}", env!("CARGO_PKG_VERSION"))
        );
        assert!(!config.allow_insecure_http());
        assert_eq!(config.retry(), &RetryConfig::default());
    }

    #[test]
    fn blank_primary_env_value_falls_back_to_secondary_key() {
        let config = build_with_fake_env(
            ConfigBuilder::new(),
            &[("MP_API_KEY", "   "), ("PMG_MAPI_KEY", "env-secondary")],
        )
        .expect("blank primary env key should fall back to secondary");

        assert_eq!(config.api_key(), "env-secondary");
    }

    #[test]
    fn http_base_url_is_rejected_by_default() {
        let error = ConfigBuilder::new()
            .api_key("test-api-key")
            .base_url("http://example.com")
            .build()
            .expect_err("http should be rejected without opt-in");

        assert!(matches!(error, MpApiError::InsecureBaseUrlNotAllowed));
    }

    #[test]
    fn http_base_url_is_allowed_for_localhost_when_opted_in() {
        let config = ConfigBuilder::new()
            .api_key("test-api-key")
            .base_url("http://localhost:8080/mp")
            .allow_insecure_http(true)
            .build()
            .expect("http should be accepted for local test endpoints");

        assert_eq!(config.base_url().as_str(), "http://localhost:8080/mp/");
    }

    #[test]
    fn http_base_url_is_rejected_for_remote_hosts_even_when_opted_in() {
        let error = ConfigBuilder::new()
            .api_key("test-api-key")
            .base_url("http://example.com")
            .allow_insecure_http(true)
            .build()
            .expect_err("remote http should be rejected even when opted in");

        assert!(matches!(error, MpApiError::InsecureBaseUrlNotAllowed));
    }

    #[test]
    fn non_http_schemes_are_rejected() {
        let error = ConfigBuilder::new()
            .api_key("test-api-key")
            .base_url("ftp://example.com")
            .allow_insecure_http(true)
            .build()
            .expect_err("non-http scheme should be rejected");

        assert!(matches!(error, MpApiError::InvalidBaseUrl));
    }

    #[test]
    fn invalid_url_string_returns_invalid_base_url() {
        let error = ConfigBuilder::new()
            .api_key("test-api-key")
            .base_url("not a url")
            .build()
            .expect_err("invalid URL should fail");

        assert!(matches!(error, MpApiError::InvalidBaseUrl));
    }

    #[test]
    fn base_url_with_embedded_credentials_is_rejected() {
        let error = ConfigBuilder::new()
            .api_key("test-api-key")
            .base_url("https://user:pass@example.com")
            .build()
            .expect_err("credential-bearing URL should be rejected");

        assert!(matches!(error, MpApiError::InvalidBaseUrl));
    }

    #[test]
    fn zero_concurrency_is_rejected() {
        let error = ConfigBuilder::new()
            .api_key("test-api-key")
            .concurrency(0)
            .build()
            .expect_err("zero concurrency should fail");

        assert!(matches!(
            error,
            MpApiError::InvalidConfiguration {
                field: "concurrency",
                ..
            }
        ));
    }

    #[test]
    fn zero_qps_limit_is_rejected() {
        let error = ConfigBuilder::new()
            .api_key("test-api-key")
            .qps_limit(0)
            .build()
            .expect_err("zero qps should fail");

        assert!(matches!(
            error,
            MpApiError::InvalidConfiguration {
                field: "qps_limit",
                ..
            }
        ));
    }

    #[test]
    fn empty_builder_api_key_is_rejected() {
        let error = ConfigBuilder::new()
            .api_key("   ")
            .build()
            .expect_err("blank api key should fail");

        assert!(matches!(
            error,
            MpApiError::InvalidConfiguration {
                field: "api_key",
                ..
            }
        ));
    }

    #[test]
    fn empty_user_agent_is_rejected() {
        let error = ConfigBuilder::new()
            .api_key("test-api-key")
            .user_agent("  ")
            .build()
            .expect_err("blank user-agent should fail");

        assert!(matches!(
            error,
            MpApiError::InvalidConfiguration {
                field: "user_agent",
                ..
            }
        ));
    }

    #[test]
    fn debug_output_redacts_api_key() {
        let config = build_with_explicit_api_key().expect("explicit key should build");
        let rendered = format!("{config:?}");

        assert!(rendered.contains("<redacted>"));
        assert!(!rendered.contains("test-api-key"));
    }

    #[test]
    fn builder_debug_output_redacts_api_key() {
        let builder = ConfigBuilder::new().api_key("builder-secret");
        let rendered = format!("{builder:?}");

        assert!(rendered.contains("<redacted>"));
        assert!(!rendered.contains("builder-secret"));
    }

    #[test]
    fn debug_output_redacts_base_url_query_and_fragment() {
        let config = ConfigBuilder::new()
            .api_key("test-api-key")
            .base_url("https://example.com/api?token=super-secret#frag")
            .build()
            .expect("config with query and fragment should build");
        let rendered = format!("{config:?}");

        assert!(rendered.contains("https://example.com/api/"));
        assert!(!rendered.contains("super-secret"));
        assert!(!rendered.contains("token="));
        assert!(!rendered.contains("#frag"));
    }

    #[test]
    fn builder_debug_output_redacts_invalid_base_url_input() {
        let builder = ConfigBuilder::new()
            .api_key("builder-secret")
            .base_url("://user:pass@example.com?token=super-secret");
        let rendered = format!("{builder:?}");

        assert!(rendered.contains("<redacted-invalid-url>"));
        assert!(!rendered.contains("user:pass"));
        assert!(!rendered.contains("super-secret"));
    }

    #[test]
    fn no_timeout_disables_timeout() {
        let config = ConfigBuilder::new()
            .api_key("test-api-key")
            .no_timeout()
            .build()
            .expect("no_timeout should build");

        assert_eq!(config.timeout(), None);
    }

    #[test]
    fn custom_values_override_defaults() {
        let retry = RetryConfig::new(5, Duration::from_secs(1), Duration::from_secs(8))
            .expect("valid retry config should build");
        let config = ConfigBuilder::new()
            .api_key("test-api-key")
            .base_url("https://example.com/mp")
            .timeout(Duration::from_secs(10))
            .concurrency(8)
            .qps_limit(50)
            .user_agent("custom-agent/1.0")
            .allow_insecure_http(true)
            .retry(retry.clone())
            .build()
            .expect("custom config should build");

        assert_eq!(config.base_url().as_str(), "https://example.com/mp/");
        assert_eq!(config.timeout(), Some(Duration::from_secs(10)));
        assert_eq!(config.concurrency(), 8);
        assert_eq!(config.qps_limit(), 50);
        assert_eq!(config.user_agent(), "custom-agent/1.0");
        assert!(config.allow_insecure_http());
        assert_eq!(config.retry(), &retry);
    }

    #[test]
    fn dotenv_loader_ignores_missing_file() {
        let result =
            load_dotenv_if_present(|| Err(dotenvy::Error::Io(io::ErrorKind::NotFound.into())));

        assert!(result.is_ok());
    }

    #[test]
    fn dotenv_loader_surfaces_non_missing_errors() {
        let error =
            load_dotenv_if_present(|| Err(dotenvy::Error::LineParse("BAD=LINE".to_string(), 0)))
                .expect_err("parse errors should be surfaced");

        assert!(matches!(
            error,
            MpApiError::InvalidConfiguration { field: ".env", .. }
        ));
    }
}
