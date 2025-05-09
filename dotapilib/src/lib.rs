use builder::RequestBuilder;

mod builder;
mod parser;
#[cfg(test)]
mod tests;

// TODO: delete
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// TODO: remove lint flag
#[allow(unused)]
pub fn parse_api_string(content: &str) -> Result<RequestBuilder, String> {
    let b = RequestBuilder::new("name".to_string(), "url".to_string());
    return Ok(b);
}
