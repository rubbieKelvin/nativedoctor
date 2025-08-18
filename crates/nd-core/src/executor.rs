use std::collections::HashMap;

use crate::models::requestroot::{HttpMethod, RequestRootModel};

#[derive(Default)]
pub struct Executor {
    client: reqwest::blocking::Client,
    env: HashMap<String, String>,     // env from .env files
    dyn_env: HashMap<String, String>, // runtime variables
}

impl Executor {
    pub fn variables(&self) -> HashMap<String, String> {
        let mut env = self.env.clone();
        // extend with the content of dynamic env
        env.extend(self.dyn_env.iter().map(|(k, v)| (k.clone(), v.clone())));
        return env;
    }

    // Takes a string and returns an evaluated string from the env store
    pub fn build_string<S: AsRef<str>>(&self, string: S) -> String {
        let string = string.as_ref();
        // TODO: work to do here
        return string.to_string();
    }

    // turns the request model into a callable request
    pub fn build_request(
        &self,
        model: RequestRootModel,
    ) -> Result<reqwest::blocking::RequestBuilder, anyhow::Error> {
        let url = self.build_string(&model.url);

        let request = match model.method {
            HttpMethod::Get => self.client.get(url),
            HttpMethod::Post => self.client.post(url),
            HttpMethod::Delete => self.client.delete(url),
            HttpMethod::Head => self.client.head(url),
            HttpMethod::Put => self.client.put(url),
            HttpMethod::Patch => self.client.patch(url),
            _ => {
                anyhow::bail!("Invalid method. {} is not supported.", model.method)
            }
        };
        
        return Ok(request);
    }
}
