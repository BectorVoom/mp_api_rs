use std::time::Duration;

use crate::error::MpApiError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetryConfig {
    max_retries: u32,
    initial_backoff: Duration,
    max_backoff: Duration,
}

impl RetryConfig {
    pub fn new(
        max_retries: u32,
        initial_backoff: Duration,
        max_backoff: Duration,
    ) -> Result<Self, MpApiError> {
        validate_backoff("retry.initial_backoff", initial_backoff)?;
        validate_backoff("retry.max_backoff", max_backoff)?;

        if initial_backoff > max_backoff {
            return Err(MpApiError::InvalidConfiguration {
                field: "retry",
                message: "initial_backoff must be less than or equal to max_backoff".to_string(),
            });
        }

        Ok(Self {
            max_retries,
            initial_backoff,
            max_backoff,
        })
    }

    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    pub fn initial_backoff(&self) -> Duration {
        self.initial_backoff
    }

    pub fn max_backoff(&self) -> Duration {
        self.max_backoff
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(200),
            max_backoff: Duration::from_secs(2),
        }
    }
}

fn validate_backoff(field: &'static str, duration: Duration) -> Result<(), MpApiError> {
    if duration.is_zero() {
        return Err(MpApiError::InvalidConfiguration {
            field,
            message: "must be greater than 0".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::error::MpApiError;

    use super::RetryConfig;

    #[test]
    fn default_values_match_spec() {
        let config = RetryConfig::default();

        assert_eq!(config.max_retries(), 3);
        assert_eq!(config.initial_backoff(), Duration::from_millis(200));
        assert_eq!(config.max_backoff(), Duration::from_secs(2));
    }

    #[test]
    fn retry_config_rejects_zero_backoffs() {
        let error = RetryConfig::new(1, Duration::ZERO, Duration::from_secs(1))
            .expect_err("zero initial backoff should fail");

        assert!(matches!(
            error,
            MpApiError::InvalidConfiguration {
                field: "retry.initial_backoff",
                ..
            }
        ));
    }

    #[test]
    fn retry_config_rejects_inverted_backoffs() {
        let error = RetryConfig::new(1, Duration::from_secs(2), Duration::from_secs(1))
            .expect_err("initial backoff larger than max should fail");

        assert!(matches!(
            error,
            MpApiError::InvalidConfiguration { field: "retry", .. }
        ));
    }
}
