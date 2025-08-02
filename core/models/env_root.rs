use std::collections::HashMap;

pub struct EnvironmentRoot {
    name: String,
    variables: HashMap<String, EnvVariable>,
}

pub struct EnvVariable {
    value: String,
    secret: bool,
}
