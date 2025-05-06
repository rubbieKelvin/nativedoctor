use std::collections::HashMap;

use regex::Regex;

use crate::types::HttpMethod;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Env(HashMap<String, String>),
    Request {
        name: String,
        method: Option<HttpMethod>,
        url: Option<String>,
    },
}

enum InblockProcessingState {
    Working,
    Done,
    Error(String),
}

fn process_env_token_line(token: &mut Token, line: &str) -> InblockProcessingState {
    if line == "@end" {
        return InblockProcessingState::Done;
    }
    // we'd typicaclly only have <name> <eq> <value> in env blocks, so every line should look like this
    // names should be alpha numeric, not starting with numbers, just like regular programing vars
    // tho names can include an env specifiyer, followed after a dot:
    // eg - base_url, base_url.dev, base_url.prod
    // Pattern explanation:
    // - name: [a-zA-Z_]\w*
    // - optional: .env, where env is [a-zA-Z_]\w*
    // - = value
    let pattern =
        r"^\s*(?P<name>[a-zA-Z_]\w*)(?:\.(?P<env>[a-zA-Z_]\w*))?\s*=\s*(?P<value>.+?)\s*$";
    let re = Regex::new(pattern).unwrap();

    if let Some(matches) = re.captures(line) {
        let name = matches.name("name").unwrap().as_str().to_string();
        let env = matches.name("env").map(|m| m.as_str().to_string());
        let value = matches.name("value").unwrap().as_str().to_string();

        match token {
            Token::Env(map) => {
                let key = match env {
                    Some(env) => format!("{name}.{env}"),
                    None => name.clone(),
                };

                // if map.contains_key(&key) {
                //     // can trow a warning here
                // }

                map.insert(key, value.clone());
            }
            _ => {
                return InblockProcessingState::Error(
                    "Invalid token, token should be Token::Env".to_string(),
                );
            }
        };
    } else {
        return InblockProcessingState::Error("Invalid syntax for env".to_string());
    }
    return InblockProcessingState::Working;
}

#[allow(unused)]
pub fn parse_tokens(content: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_token: Option<Token> = None;

    // might need to replce \n for os equiv;
    let lines = content.split("\n");
    for i in lines {
        let line = i.trim();

        match &mut current_token {
            Some(token) => {
                let state = match token {
                    Token::Env(_) => process_env_token_line(token, line),
                    _ => {
                        unreachable!()
                    }
                };

                // add current token to tokens if done
                match state {
                    InblockProcessingState::Done => {
                        let token = current_token.take().unwrap();
                        tokens.push(token);
                    }
                    InblockProcessingState::Error(value) => {
                        return Err(value);
                    }
                    _ => {}
                };
            }
            None => {
                if line.len() != 0 {
                    if line == "@env" {
                        current_token = Some(Token::Env(HashMap::new()))
                    }
                }
            }
        };
    }

    assert!(current_token.is_none());
    println!("{:?}", &tokens);
    return Ok(tokens);
}
