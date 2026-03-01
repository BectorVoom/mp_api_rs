use std::collections::BTreeMap;

use crate::error::MpApiError;

use super::pagination::ToQueryPairs;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExtraQueryParams(pub BTreeMap<String, String>);

impl ToQueryPairs for ExtraQueryParams {
    fn to_query_pairs(&self) -> Result<Vec<(String, String)>, MpApiError> {
        if let Some(reserved_key) = self.0.keys().find(|key| is_reserved_query_key(key)) {
            return Err(MpApiError::InvalidQueryParameters {
                details: format!(
                    "reserved query parameter `{reserved_key}` must be provided through typed query builders"
                ),
            });
        }

        Ok(self
            .0
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect())
    }
}

fn is_reserved_query_key(key: &str) -> bool {
    matches!(
        key,
        "_page" | "_per_page" | "_skip" | "_limit" | "_fields" | "_all_fields"
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{ExtraQueryParams, ToQueryPairs};

    #[test]
    fn extra_query_params_serialize_in_btree_order() {
        let extra = ExtraQueryParams(BTreeMap::from([
            ("z_last".to_string(), "3".to_string()),
            ("a_first".to_string(), "1".to_string()),
            ("m_middle".to_string(), "2".to_string()),
        ]));

        assert_eq!(
            extra
                .to_query_pairs()
                .expect("extra query params should serialize"),
            vec![
                ("a_first".to_string(), "1".to_string()),
                ("m_middle".to_string(), "2".to_string()),
                ("z_last".to_string(), "3".to_string()),
            ]
        );
    }

    #[test]
    fn empty_string_values_are_preserved() {
        let extra = ExtraQueryParams(BTreeMap::from([("empty".to_string(), "".to_string())]));

        assert_eq!(
            extra
                .to_query_pairs()
                .expect("empty values should serialize"),
            vec![("empty".to_string(), "".to_string())]
        );
    }

    #[test]
    fn reserved_keys_are_rejected() {
        let extra = ExtraQueryParams(BTreeMap::from([("_page".to_string(), "99".to_string())]));

        assert!(matches!(
            extra.to_query_pairs(),
            Err(crate::error::MpApiError::InvalidQueryParameters { .. })
        ));
    }

    #[test]
    fn non_reserved_underscore_keys_are_allowed() {
        let extra = ExtraQueryParams(BTreeMap::from([(
            "_custom_filter".to_string(),
            "allowed".to_string(),
        )]));

        assert_eq!(
            extra
                .to_query_pairs()
                .expect("non-reserved underscore keys should serialize"),
            vec![("_custom_filter".to_string(), "allowed".to_string())]
        );
    }
}
