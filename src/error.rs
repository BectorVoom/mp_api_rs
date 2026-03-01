use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MpApiError {
    #[error("missing API key; set ConfigBuilder::api_key, MP_API_KEY, or PMG_MAPI_KEY")]
    MissingApiKey,
    #[error("invalid query parameters: {details}")]
    InvalidQueryParameters { details: String },
    #[error("invalid pagination parameters: {details}")]
    InvalidPaginationParameters { details: String },
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
            MpApiError::InvalidQueryParameters {
                details: "reserved query parameter `_page` must be provided through typed query builders"
                    .to_string(),
            },
            MpApiError::InvalidPaginationParameters {
                details: "cannot combine page-based pagination (_page/_per_page) with offset-based pagination (_skip/_limit)"
                    .to_string(),
            },
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

    #[test]
    fn invalid_pagination_parameters_display_includes_details() {
        let error = MpApiError::InvalidPaginationParameters {
            details: "page and skip cannot be combined".to_string(),
        };

        assert!(
            error
                .to_string()
                .contains("page and skip cannot be combined")
        );
    }

    #[test]
    fn invalid_query_parameters_display_includes_details() {
        let error = MpApiError::InvalidQueryParameters {
            details:
                "reserved query parameter `_page` must be provided through typed query builders"
                    .to_string(),
        };

        assert!(
            error
                .to_string()
                .contains("reserved query parameter `_page`")
        );
    }
}
