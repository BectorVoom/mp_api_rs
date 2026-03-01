use crate::error::MpApiError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub skip: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct NormalizedPagination {
    page: Option<u32>,
    per_page: Option<u32>,
    skip: Option<u32>,
    limit: Option<u32>,
}

pub trait ToQueryPairs {
    fn to_query_pairs(&self) -> Result<Vec<(String, String)>, MpApiError>;
}

impl Pagination {
    pub(crate) fn validate_and_normalize(&self) -> Result<NormalizedPagination, MpApiError> {
        let has_page_mode = self.page.is_some() || self.per_page.is_some();
        let has_offset_mode = self.skip.is_some() || self.limit.is_some();

        if has_page_mode && has_offset_mode {
            return Err(MpApiError::InvalidPaginationParameters {
                details:
                    "cannot combine page-based pagination (_page/_per_page) with offset-based pagination (_skip/_limit)"
                        .to_string(),
            });
        }

        if has_page_mode {
            return Ok(NormalizedPagination {
                page: self.page,
                per_page: self.per_page.map(|value| value.min(1_000)),
                skip: None,
                limit: None,
            });
        }

        Ok(NormalizedPagination {
            page: None,
            per_page: None,
            skip: self.skip,
            limit: self.limit.map(|value| value.min(1_000)),
        })
    }
}

impl ToQueryPairs for Pagination {
    fn to_query_pairs(&self) -> Result<Vec<(String, String)>, MpApiError> {
        self.validate_and_normalize()?.to_query_pairs()
    }
}

impl ToQueryPairs for NormalizedPagination {
    fn to_query_pairs(&self) -> Result<Vec<(String, String)>, MpApiError> {
        let mut query_pairs = Vec::new();

        if let Some(page) = self.page {
            query_pairs.push(("_page".to_string(), page.to_string()));
        }

        if let Some(per_page) = self.per_page {
            query_pairs.push(("_per_page".to_string(), per_page.to_string()));
        }

        if let Some(skip) = self.skip {
            query_pairs.push(("_skip".to_string(), skip.to_string()));
        }

        if let Some(limit) = self.limit {
            query_pairs.push(("_limit".to_string(), limit.to_string()));
        }

        Ok(query_pairs)
    }
}

#[cfg(test)]
mod tests {
    use super::{Pagination, ToQueryPairs};
    use crate::error::MpApiError;

    #[test]
    fn page_only_serializes_to_page_query_pair() {
        let pagination = Pagination {
            page: Some(2),
            ..Pagination::default()
        };

        assert_eq!(
            pagination
                .to_query_pairs()
                .expect("page pagination should serialize"),
            vec![("_page".to_string(), "2".to_string())]
        );
    }

    #[test]
    fn per_page_only_serializes_to_per_page_query_pair() {
        let pagination = Pagination {
            per_page: Some(50),
            ..Pagination::default()
        };

        assert_eq!(
            pagination
                .to_query_pairs()
                .expect("per-page pagination should serialize"),
            vec![("_per_page".to_string(), "50".to_string())]
        );
    }

    #[test]
    fn page_and_per_page_serialize_in_stable_order() {
        let pagination = Pagination {
            page: Some(3),
            per_page: Some(25),
            ..Pagination::default()
        };

        assert_eq!(
            pagination
                .to_query_pairs()
                .expect("page-based pagination should serialize"),
            vec![
                ("_page".to_string(), "3".to_string()),
                ("_per_page".to_string(), "25".to_string()),
            ]
        );
    }

    #[test]
    fn offset_only_serializes_to_skip_and_limit_pairs() {
        let pagination = Pagination {
            skip: Some(10),
            limit: Some(25),
            ..Pagination::default()
        };

        assert_eq!(
            pagination
                .to_query_pairs()
                .expect("offset-based pagination should serialize"),
            vec![
                ("_skip".to_string(), "10".to_string()),
                ("_limit".to_string(), "25".to_string()),
            ]
        );
    }

    #[test]
    fn per_page_is_clamped_to_one_thousand() {
        let pagination = Pagination {
            page: Some(2),
            per_page: Some(2_001),
            ..Pagination::default()
        };

        assert_eq!(
            pagination
                .to_query_pairs()
                .expect("page-based pagination should serialize"),
            vec![
                ("_page".to_string(), "2".to_string()),
                ("_per_page".to_string(), "1000".to_string()),
            ]
        );
    }

    #[test]
    fn limit_is_clamped_to_one_thousand() {
        let pagination = Pagination {
            skip: Some(20),
            limit: Some(5_000),
            ..Pagination::default()
        };

        assert_eq!(
            pagination
                .to_query_pairs()
                .expect("offset-based pagination should serialize"),
            vec![
                ("_skip".to_string(), "20".to_string()),
                ("_limit".to_string(), "1000".to_string()),
            ]
        );
    }

    #[test]
    fn page_and_skip_conflict_is_rejected() {
        let pagination = Pagination {
            page: Some(1),
            skip: Some(10),
            ..Pagination::default()
        };

        assert!(matches!(
            pagination.to_query_pairs(),
            Err(MpApiError::InvalidPaginationParameters { .. })
        ));
    }

    #[test]
    fn per_page_and_limit_conflict_is_rejected() {
        let pagination = Pagination {
            per_page: Some(25),
            limit: Some(50),
            ..Pagination::default()
        };

        assert!(matches!(
            pagination.to_query_pairs(),
            Err(MpApiError::InvalidPaginationParameters { .. })
        ));
    }

    #[test]
    fn all_page_and_offset_fields_conflict_is_rejected() {
        let pagination = Pagination {
            page: Some(1),
            per_page: Some(10),
            skip: Some(20),
            limit: Some(30),
        };

        assert!(matches!(
            pagination.to_query_pairs(),
            Err(MpApiError::InvalidPaginationParameters { .. })
        ));
    }

    #[test]
    fn page_and_limit_conflict_is_rejected() {
        let pagination = Pagination {
            page: Some(1),
            per_page: None,
            limit: Some(5),
            ..Pagination::default()
        };

        assert!(matches!(
            pagination.to_query_pairs(),
            Err(MpApiError::InvalidPaginationParameters { .. })
        ));
    }

    #[test]
    fn per_page_and_skip_conflict_is_rejected() {
        let pagination = Pagination {
            per_page: Some(25),
            skip: Some(50),
            ..Pagination::default()
        };

        assert!(matches!(
            pagination.to_query_pairs(),
            Err(MpApiError::InvalidPaginationParameters { .. })
        ));
    }
}
