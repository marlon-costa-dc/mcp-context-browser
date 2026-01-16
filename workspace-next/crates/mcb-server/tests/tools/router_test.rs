//! Tool Router Tests

use mcb_server::tools::router::{parse_args, SearchCodeArgs};
use rmcp::model::CallToolRequestParam;
use serde_json::json;
use std::borrow::Cow;

#[test]
fn test_parse_args_valid() {
    let request = CallToolRequestParam {
        name: Cow::Borrowed("search_code"),
        arguments: Some(
            json!({
                "query": "test query",
                "limit": 10
            })
            .as_object()
            .cloned()
            .unwrap(),
        ),
    };

    let args: SearchCodeArgs = parse_args(&request).expect("should parse args");
    assert_eq!(args.query, "test query");
    assert_eq!(args.limit, 10);
}

#[test]
fn test_parse_args_default_limit() {
    let request = CallToolRequestParam {
        name: Cow::Borrowed("search_code"),
        arguments: Some(
            json!({
                "query": "test query"
            })
            .as_object()
            .cloned()
            .unwrap(),
        ),
    };

    let args: SearchCodeArgs = parse_args(&request).expect("should parse args");
    assert_eq!(args.query, "test query");
    assert!(args.limit > 0);
}
