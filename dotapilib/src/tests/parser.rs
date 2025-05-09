use crate::parser;

#[test]
fn test_parse_env() {
    let content = r#"
        @env
        base_url = https://google.com
        base_url.dev = http://localhost:3000
        @end
    "#;
    let tokens = parser::parse_tokens(content).unwrap();
    assert_eq!(tokens.len(), 1);
}

#[test]
fn test_parse_request() {
    let content = r#"
    "#;
}
