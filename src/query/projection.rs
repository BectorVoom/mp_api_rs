use crate::error::MpApiError;

use super::pagination::ToQueryPairs;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Projection {
    pub fields: Option<Vec<String>>,
    pub all_fields: bool,
}

impl ToQueryPairs for Projection {
    fn to_query_pairs(&self) -> Result<Vec<(String, String)>, MpApiError> {
        let mut query_pairs = Vec::new();

        if let Some(fields) = &self.fields {
            let filtered_fields = fields
                .iter()
                .map(|field| field.trim())
                .filter(|field| !field.is_empty())
                .collect::<Vec<_>>();

            if !filtered_fields.is_empty() {
                query_pairs.push(("_fields".to_string(), filtered_fields.join(",")));
            }
        }

        if self.all_fields {
            query_pairs.push(("_all_fields".to_string(), "true".to_string()));
        }

        Ok(query_pairs)
    }
}

#[cfg(test)]
mod tests {
    use super::{Projection, ToQueryPairs};

    #[test]
    fn fields_serialize_to_comma_separated_list() {
        let projection = Projection {
            fields: Some(vec![
                "material_id".to_string(),
                "formula_pretty".to_string(),
            ]),
            all_fields: false,
        };

        assert_eq!(
            projection
                .to_query_pairs()
                .expect("projection fields should serialize"),
            vec![(
                "_fields".to_string(),
                "material_id,formula_pretty".to_string(),
            )]
        );
    }

    #[test]
    fn all_fields_serializes_to_true_flag() {
        let projection = Projection {
            fields: None,
            all_fields: true,
        };

        assert_eq!(
            projection
                .to_query_pairs()
                .expect("all-fields projection should serialize"),
            vec![("_all_fields".to_string(), "true".to_string())]
        );
    }

    #[test]
    fn fields_and_all_fields_serialize_in_stable_order() {
        let projection = Projection {
            fields: Some(vec![
                "material_id".to_string(),
                "formula_pretty".to_string(),
            ]),
            all_fields: true,
        };

        assert_eq!(
            projection
                .to_query_pairs()
                .expect("combined projection should serialize"),
            vec![
                (
                    "_fields".to_string(),
                    "material_id,formula_pretty".to_string(),
                ),
                ("_all_fields".to_string(), "true".to_string()),
            ]
        );
    }

    #[test]
    fn fields_are_trimmed_and_empty_entries_are_removed() {
        let projection = Projection {
            fields: Some(vec![
                " material_id ".to_string(),
                " ".to_string(),
                "formula_pretty".to_string(),
            ]),
            all_fields: false,
        };

        assert_eq!(
            projection
                .to_query_pairs()
                .expect("trimmed projection should serialize"),
            vec![(
                "_fields".to_string(),
                "material_id,formula_pretty".to_string(),
            )]
        );
    }

    #[test]
    fn empty_filtered_fields_omit_fields_entry() {
        let projection = Projection {
            fields: Some(vec![" ".to_string(), "".to_string()]),
            all_fields: false,
        };

        assert_eq!(
            projection
                .to_query_pairs()
                .expect("empty filtered fields should serialize"),
            Vec::<(String, String)>::new()
        );
    }
}
