use std::collections::BTreeMap;

use mp_api_rs::{ExtraQueryParams, MpApiError, Pagination, Projection, ToQueryPairs};

#[test]
fn public_query_api_serializes_pagination_projection_and_extra_params() {
    let pagination = Pagination {
        page: Some(2),
        per_page: Some(2_001),
        skip: None,
        limit: None,
    };
    let projection = Projection {
        fields: Some(vec![
            "material_id".to_string(),
            "formula_pretty".to_string(),
        ]),
        all_fields: true,
    };
    let extra = ExtraQueryParams(BTreeMap::from([
        ("chemsys".to_string(), "Li-Fe-O".to_string()),
        ("deprecated".to_string(), "false".to_string()),
    ]));

    assert_eq!(
        pagination
            .to_query_pairs()
            .expect("public pagination API should serialize"),
        vec![
            ("_page".to_string(), "2".to_string()),
            ("_per_page".to_string(), "1000".to_string()),
        ]
    );
    assert_eq!(
        projection
            .to_query_pairs()
            .expect("public projection API should serialize"),
        vec![
            (
                "_fields".to_string(),
                "material_id,formula_pretty".to_string(),
            ),
            ("_all_fields".to_string(), "true".to_string()),
        ]
    );
    assert_eq!(
        extra
            .to_query_pairs()
            .expect("public extra query API should serialize"),
        vec![
            ("chemsys".to_string(), "Li-Fe-O".to_string()),
            ("deprecated".to_string(), "false".to_string()),
        ]
    );
}

#[test]
fn public_query_api_rejects_mixed_pagination_modes() {
    let pagination = Pagination {
        page: Some(4),
        per_page: None,
        skip: None,
        limit: Some(99),
    };

    assert!(matches!(
        pagination.to_query_pairs(),
        Err(mp_api_rs::MpApiError::InvalidPaginationParameters { .. })
    ));
}

#[test]
fn public_query_api_rejects_reserved_keys_in_extra_query_params() {
    let extra = ExtraQueryParams(BTreeMap::from([("_skip".to_string(), "10".to_string())]));

    assert!(matches!(
        extra.to_query_pairs(),
        Err(MpApiError::InvalidQueryParameters { .. })
    ));
}
