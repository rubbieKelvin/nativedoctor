use crate::parser;

#[test]
fn test_parse_env() {
    let content = r#"@env
base_url = "https://google.com"
@end
"#;

    let api_stuct_result = parser::parse_api_content(content);

    if let Err(e) = &api_stuct_result {
        eprintln!("Error occured: {e}");
    }

    let api_struct = api_stuct_result.unwrap();
    assert_eq!(api_struct.blocks.len(), 1);

    let first_block = api_struct.blocks.get(0).unwrap();

    if let parser::types::Blocks::Env(block) = first_block {
        let variable_keys = block
            .variables
            .iter()
            .map(|variable| {
                return match &variable.environment {
                    Some(env) => format!("{}.{}", variable.name, env),
                    None => variable.name.clone(),
                };
            })
            .collect::<Vec<String>>();

        assert!(variable_keys.contains(&"base_url".to_string()));
        assert!(variable_keys.contains(&"base_url.dev".to_string()));
    } else {
        assert!(false, "First block should have been an env block");
    }
}
