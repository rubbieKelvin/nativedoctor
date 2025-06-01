use std::collections::HashMap;

use rhai::{CustomType, Dynamic, Engine, EvalAltResult, Position, Scope, TypeBuilder};
use tracing::warn;

#[derive(Clone)]
pub struct ScriptingEnviroment {
    pub env_variables: HashMap<String, Dynamic>,
}

impl ScriptingEnviroment {
    fn from_yaml_value(obj: &HashMap<String, serde_yaml::Value>) -> Self {
        let env_variables = obj
            .iter()
            .filter_map(|(k, v)| {
                yaml_value_to_rhai_dynamic(v.clone())
                    .ok()
                    .map(|d| (k.clone(), d))
            })
            .collect::<HashMap<String, Dynamic>>();
        return ScriptingEnviroment { env_variables };
    }

    fn get_env(&self, key: &str) -> Option<Dynamic> {
        return match self.env_variables.get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        };
    }

    fn set_env(&mut self, key: String, value: Dynamic) {
        self.env_variables.insert(key, value);
    }

    fn clear(&mut self) {
        self.env_variables.clear();
    }
}

impl CustomType for ScriptingEnviroment {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ScriptingEnvironment")
            .with_fn("set", Self::set_env)
            .with_fn("get", Self::get_env)
            .with_fn("clear", Self::clear);
    }
}

pub struct RhaiScripting {
    pub engine: Engine,
}

impl RhaiScripting {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        // Register functions
        engine
            .register_fn("log", |msg: &str| {
                println!("[Rhai] {}", msg);
            })
            .register_fn(
                "assert",
                |condition: bool| -> Result<(), Box<EvalAltResult>> {
                    if !condition {
                        Err(EvalAltResult::ErrorTerminated("Assertion failed".into(), Position::NONE).into())
                    } else {
                        Ok(())
                    }
                },
            )
            .register_fn(
                "assert",
                |condition: bool, msg: &str| -> Result<(), Box<EvalAltResult>> {
                    if !condition {
                        Err(EvalAltResult::ErrorTerminated(
                            format!("Assertion failed: {}", msg).into(),
                            Position::NONE,
                        )
                        .into())
                    } else {
                        Ok(())
                    }
                },
            );

        // Register types
        engine.build_type::<ScriptingEnviroment>();

        return RhaiScripting { engine };
    }

    pub fn run(
        &self,
        script: &str,
        env: &mut HashMap<String, serde_yaml::Value>,
    ) -> anyhow::Result<()> {
        let mut scope = Scope::new();
        let shared_env = ScriptingEnviroment::from_yaml_value(env);

        scope.push("env", shared_env);
        self.engine
            .run_with_scope(&mut scope, script)
            .map_err(|e| anyhow::anyhow!("Script execution failed: {}", e))?;
        return Ok(());
    }
}

fn yaml_value_to_rhai_dynamic(value: serde_yaml::Value) -> Result<Dynamic, Box<EvalAltResult>> {
    return match value {
        serde_yaml::Value::Bool(v) => Ok(Dynamic::from_bool(v)),
        serde_yaml::Value::Null => Ok(Dynamic::UNIT),
        serde_yaml::Value::Number(v) => {
            if v.is_i64() {
                Ok(Dynamic::from_int(v.as_i64().unwrap()))
            } else if v.is_f64() {
                Ok(Dynamic::from_float(v.as_f64().unwrap()))
            } else {
                warn!("Not handling other integer types. representing as unit");
                Ok(Dynamic::UNIT)
            }
        }
        serde_yaml::Value::String(s) => Ok(Dynamic::from(s)),
        serde_yaml::Value::Sequence(seq) => {
            let res = seq
                .iter()
                .map(|v| yaml_value_to_rhai_dynamic(v.clone()))
                .collect::<Result<Vec<Dynamic>, Box<EvalAltResult>>>();
            Ok(Dynamic::from_array(res?))
        }
        serde_yaml::Value::Mapping(mp) => {
            let mut object = rhai::Map::new();
            for (k, v) in mp {
                let key = match k {
                    serde_yaml::Value::String(s) => s,
                    _ => {
                        warn!("Converting value to string while resolving rhai map");
                        k.as_str().unwrap().to_string()
                    }
                };
                object.insert(key.into(), yaml_value_to_rhai_dynamic(v)?);
            }
            Ok(Dynamic::from_map(object))
        }
        // Handle other Value variants if necessary
        _ => {
            eprintln!("Warning: Unsupported YAML value type: {:?}", value);
            Ok(Dynamic::UNIT) // Default to unit for unsupported types
        }
    };
}
