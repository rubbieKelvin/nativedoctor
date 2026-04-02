//! Custom [`rhai::ModuleResolver`]: `.rhai` modules and request files (`json`/`yaml`/`yml`).
//!
//! Imported request modules expose **`invoke`** (not `call`): Rhai reserves the identifier `call`
//! in qualified paths such as `module::call(...)`, so scripts should use `my_req::invoke(#{ key: "v" })`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rhai::{EvalAltResult, Module, ModuleResolver, Position, Scope, Shared};

use super::utils::json_to_dynamic;
use crate::env::RuntimeEnv;
use crate::error::Error as NdError;
use crate::execute::types::ExecutionResult;
use crate::model::request::RequestFile;
use crate::utils::path::resolve_file_path;

/// CLI-aligned options for HTTP invoked from Rhai `import` request modules.
#[derive(Debug, Clone, Default)]
pub struct RhaiScriptRunOptions {
    /// When true, `invoke()` does not perform network I/O (expand only; see return map).
    pub no_network_io: bool,
}

/// Resolves `import "path"` relative to the importing script (or the main script directory).
pub struct NativeImportResolver {
    main_script_dir: PathBuf,
    env: Arc<RuntimeEnv>,
    options: RhaiScriptRunOptions,
    cache: Mutex<HashMap<PathBuf, Shared<Module>>>,
}

impl NativeImportResolver {
    pub fn new(main_script: &Path, env: Arc<RuntimeEnv>, options: RhaiScriptRunOptions) -> Self {
        let main_script_dir = main_script
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));

        Self {
            main_script_dir,
            env,
            options,
            cache: Mutex::new(HashMap::new()),
        }
    }

    fn resolve_target_file(
        &self,
        source: Option<&str>,
        path: &str,
        pos: Position,
    ) -> Result<PathBuf, Box<EvalAltResult>> {
        let base_dir = source
            .map(Path::new)
            .and_then(|p| p.parent())
            .unwrap_or(self.main_script_dir.as_path());

        let resolved = resolve_file_path(base_dir, path);

        if resolved.is_file() {
            return Ok(resolved);
        }

        if Path::new(path).extension().is_none() {
            let with_rhai = resolve_file_path(base_dir, &format!("{path}.rhai"));
            if with_rhai.is_file() {
                return Ok(with_rhai);
            }
        }
        return Err(Box::new(EvalAltResult::ErrorModuleNotFound(
            path.to_string(),
            pos,
        )));
    }

    fn cached_or_insert(
        &self,
        key: PathBuf,
        f: impl FnOnce() -> Result<Shared<Module>, Box<EvalAltResult>>,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        {
            let guard = self.cache.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(m) = guard.get(&key) {
                return Ok(m.clone());
            }
        }
        let module = f()?;
        let mut guard = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        guard.insert(key, module.clone());
        Ok(module)
    }

    fn load_rhai_module(
        &self,
        engine: &rhai::Engine,
        file_path: PathBuf,
        path: &str,
        pos: Position,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        let ast = engine
            .compile_file_with_scope(&Scope::new(), file_path.clone())
            .map_err(|err| Box::new(EvalAltResult::ErrorInModule(path.to_string(), err, pos)))?;
        let mut ast = ast;
        ast.set_source(path);

        let module = Module::eval_ast_as_new(Scope::new(), &ast, engine)
            .map_err(|err| Box::new(EvalAltResult::ErrorInModule(path.to_string(), err, pos)))?;

        return Ok(Shared::new(module));
    }

    fn load_request_module(
        &self,
        file_path: PathBuf,
        path: &str,
        pos: Position,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        let path_owned = path.to_string();

        let doc: Arc<RequestFile> =
            Arc::new(RequestFile::from_file(&file_path).map_err(|e| nd_error(path, pos, e))?);

        let env = self.env.clone();
        let options = self.options.clone();

        let mut module = Module::new();

        let doc0 = doc.clone();
        let env0 = env.clone();
        let options0 = options.clone();
        let p0 = path_owned.clone();

        // So we can call the request (Without kwards)
        module.set_native_fn("invoke", move || {
            execute_request_call(&doc0, &env0, &options0, None, &p0, pos)
        });

        let doc1 = doc.clone();
        let env1 = env.clone();
        let options1 = options.clone();
        let p1 = path_owned;

        // Call the request with kwargs
        module.set_native_fn("invoke", move |kwargs: rhai::Map| {
            let overrides = map_to_overrides(&kwargs);
            execute_request_call(&doc1, &env1, &options1, Some(overrides), &p1, pos)
        });

        return Ok(Shared::new(module));
    }
}

fn nd_error(path: &str, pos: Position, e: NdError) -> Box<EvalAltResult> {
    return Box::new(EvalAltResult::ErrorInModule(
        path.to_string(),
        Box::new(EvalAltResult::ErrorRuntime(e.to_string().into(), pos)),
        pos,
    ));
}

fn map_to_overrides(m: &rhai::Map) -> HashMap<String, String> {
    return m
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
}

fn execution_result_to_dynamic(result: &ExecutionResult) -> rhai::Dynamic {
    let mut map = rhai::Map::new();

    map.insert("status".into(), rhai::Dynamic::from(result.status as i64));
    map.insert(
        "final_url".into(),
        rhai::Dynamic::from(result.final_url.clone()),
    );
    map.insert(
        "method".into(),
        rhai::Dynamic::from(result.method.to_string()),
    );
    map.insert(
        "duration_ms".into(),
        rhai::Dynamic::from(result.duration.as_millis() as i64),
    );

    if let Some(name) = &result.request_name {
        map.insert("request_name".into(), rhai::Dynamic::from(name.clone()));
    }

    let mut hdr = rhai::Map::new();

    for (k, v) in &result.headers {
        hdr.insert(k.clone().into(), rhai::Dynamic::from(v.clone()));
    }

    map.insert("headers".into(), rhai::Dynamic::from_map(hdr));

    if let Ok(text) = std::str::from_utf8(&result.body) {
        map.insert("body".into(), rhai::Dynamic::from(text.to_string()));
        if let Ok(j) = serde_json::from_str::<serde_json::Value>(text) {
            map.insert("json".into(), json_to_dynamic(&j));
        }
    } else {
        map.insert(
            "body".into(),
            rhai::Dynamic::from(format!("<{} bytes binary>", result.body.len())),
        );
    }

    return rhai::Dynamic::from_map(map);
}

fn execute_request_call(
    doc: &Arc<RequestFile>,
    env: &Arc<RuntimeEnv>,
    options: &RhaiScriptRunOptions,
    overrides: Option<HashMap<String, String>>,
    _import_path: &str,
    pos: Position,
) -> Result<rhai::Dynamic, Box<EvalAltResult>> {
    let overrides_ref = overrides.as_ref();

    if options.no_network_io {
        let prep = doc
            .request
            .expand_with_overrides(env, overrides_ref)
            .map_err(|e| Box::new(EvalAltResult::ErrorRuntime(e.to_string().into(), pos)))?;

        let mut map = rhai::Map::new();
        map.insert("dry_run".into(), rhai::Dynamic::from(true));
        map.insert("status".into(), rhai::Dynamic::from(0_i64));
        map.insert("final_url".into(), rhai::Dynamic::from(prep.url.clone()));
        map.insert(
            "method".into(),
            rhai::Dynamic::from(prep.method.to_string()),
        );
        return Ok(rhai::Dynamic::from_map(map));
    }

    let doc = doc.clone();
    let env = env.clone();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            doc.execute_with_overrides(env.as_ref(), overrides.as_ref())
                .await
        })
    });

    let result = result.map_err(|e: NdError| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("HTTP request failed: {e}").into(),
            pos,
        ))
    })?;

    return Ok(execution_result_to_dynamic(&result));
}

impl ModuleResolver for NativeImportResolver {
    fn resolve(
        &self,
        engine: &rhai::Engine,
        source: Option<&str>,
        path: &str,
        pos: Position,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        let resolved = self.resolve_target_file(source, path, pos)?;
        let cache_key = std::fs::canonicalize(&resolved).unwrap_or(resolved.clone());

        return self.cached_or_insert(cache_key, || {
            let ext = resolved
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            match ext.as_str() {
                "rhai" => self.load_rhai_module(engine, resolved, path, pos),
                "json" | "yaml" | "yml" => self.load_request_module(resolved, path, pos),
                _ => Err(Box::new(EvalAltResult::ErrorRuntime(
                    format!(
                        "import unsupported for extension '{ext}' (use .rhai, .json, .yaml, .yml): {}",
                        resolved.display()
                    )
                    .into(),
                    pos,
                ))),
            }
        });
    }
}
