// use pest::{error::Error, iterators::Pair, Parser};
use pest_derive::Parser;
// use types::*;

mod types;
mod yaml;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ApiParser;

// fn build_ast_from_pair(pair: Pair<Rule>) -> Result<ApiFile, String> {
//     // pair should match api_file rule.
//     return match pair.as_rule() {
//         Rule::file => {
//             let mut ast_items = Vec::new();

//             for inner_pair in pair.into_inner() {
//                 match inner_pair.as_rule() {
//                     Rule::env_block => {
//                         let env_block = build_env_block(inner_pair)?;
//                         ast_items.push(Blocks::Env(env_block));
//                     }
//                     Rule::request_block => {
//                         // todo: parse request block
//                         unreachable!();
//                     }
//                     Rule::EOI | Rule::WHITESPACE | Rule::COMMENT => {}
//                     _ => {
//                         return Err(format!(
//                             "Unexpected rule: {:?} at the top level of api_file. Text: '{}'",
//                             inner_pair.as_rule(),
//                             inner_pair.as_str()
//                         ));
//                     }
//                 };
//             }

//             return Ok(ApiFile { blocks: ast_items });
//         }
//         _ => Err(format!(
//             "Expected api file rule, found: {:?}",
//             pair.as_rule()
//         )),
//     };
// }

// fn build_env_block(pair: Pair<Rule>) -> Result<EnvBlock, String> {
//     // only env blcoks shoulf be passes here
//     if pair.as_rule() != Rule::env_block {
//         return Err(format!("Expected env block, got {:?}", pair.as_rule()));
//     }

//     let mut variables = Vec::<EnvVariable>::new();
//     for inner_pair in pair.into_inner() {
//         match inner_pair.as_rule() {
//             Rule::env_variable => {
//                 let env_var = build_env_var(inner_pair)?;
//                 variables.push(env_var);
//             }
//             Rule::WHITESPACE | Rule::COMMENT => {} // Skip these ones
//             _ => return Err(format!("Unexpected rule in env_block: {:?}", inner_pair)),
//         };
//     }

//     return Ok(EnvBlock { variables });
// }

// fn build_env_var(pair: Pair<Rule>) -> Result<EnvVariable, String> {}

// // Rule is provided by pest from the dotapi.pest language defination
// fn parse_api_content(content: &str) -> Result<ApiFile, Error<Rule>> {
//     let mut pairs = ApiParser::parse(Rule::api_file, content)?;
//     let api_content_pair = pairs.next().unwrap();

//     match build_ast_from_pair(api_content_pair) {
//         Ok(api) => return Ok(api),
//         Err(e) => {
//             eprintln!("\nError constructing AST: {e}");
//         }
//     };

//     unreachable!();
// }
