pub const PUBLIC_REQUEST_JSON_SCHEMA_URL: &str = concat!(
    "https://github.com/rubbieKelvin/nativedoctor/raw/refs/heads/main/schema/v",
    env!("CARGO_PKG_VERSION"),
    "/request.schema.json"
);

pub const PUBLIC_SEQUENCE_JSON_SCHEMA_URL: &str = concat!(
    "https://github.com/rubbieKelvin/nativedoctor/raw/refs/heads/main/schema/v",
    env!("CARGO_PKG_VERSION"),
    "/sequence.schema.json"
);

pub const PUBLIC_REQUEST_YAML_SCHEMA_URL: &str = concat!(
    "https://github.com/rubbieKelvin/nativedoctor/raw/refs/heads/main/schema/v",
    env!("CARGO_PKG_VERSION"),
    "/request.schema.yaml"
);

pub const PUBLIC_SEQUENCE_YAML_SCHEMA_URL: &str = concat!(
    "https://github.com/rubbieKelvin/nativedoctor/raw/refs/heads/main/schema/v",
    env!("CARGO_PKG_VERSION"),
    "/sequence.schema.yaml"
);
