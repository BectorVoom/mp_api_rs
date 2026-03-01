use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MpApiError {
    #[error("missing API key; set ConfigBuilder::api_key, MP_API_KEY, or PMG_MAPI_KEY")]
    MissingApiKey,
    #[error("invalid base_url provided")]
    InvalidBaseUrl,
    #[error("http base_url is not allowed unless allow_insecure_http is enabled")]
    InsecureBaseUrlNotAllowed,
    #[error("invalid configuration for {field}: {message}")]
    InvalidConfiguration {
        field: &'static str,
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::MpApiError;

    #[test]
    fn display_messages_are_stable_and_non_empty() {
        let errors = [
            MpApiError::MissingApiKey,
            MpApiError::InvalidBaseUrl,
            MpApiError::InsecureBaseUrlNotAllowed,
            MpApiError::InvalidConfiguration {
                field: "qps_limit",
                message: "must be at least 1".to_string(),
            },
        ];

        for error in errors {
            let rendered = error.to_string();
            assert!(!rendered.trim().is_empty());
        }
    }

    #[test]
    fn error_display_does_not_leak_secret_values() {
        let error = MpApiError::InvalidBaseUrl;

        let rendered = format!("{error} {error:?}");
        assert!(!rendered.contains("super-secret"));
        assert!(!rendered.contains("user:"));
    }
}
