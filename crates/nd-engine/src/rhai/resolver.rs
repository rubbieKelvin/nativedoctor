//! Custom [`rhai::ModuleResolver`]: resolves `import "path.rhai"` relative to the importing script.
//!
//! Earlier versions of nativedoctor also let scripts `import` a standalone request file directly
//! (exposing `invoke`/`stream`). Standalone request files no longer exist now that every request is
//! an OpenAPI operation (see `nd_core::model::api`); importing "a request" by `(doc, path, method)`
//! is left for a later milestone (Rhai hooks are M2 scope, not M1).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rhai::{EvalAltResult, Module, ModuleResolver, Position, Scope, Shared};

use crate::stream::Session;
use nd_core::utils::path::resolve_file_path;

/// CLI-aligned options for HTTP invoked from Rhai scripts.
#[derive(Debug, Clone, Default)]
pub struct RhaiScriptRunOptions {
    /// When true, invoked requests do not perform network I/O (expand only).
    pub no_network_io: bool,
}

/// Resolves `import "path"` relative to the importing script (or the main script directory).
pub struct NativeImportResolver {
    main_script_dir: PathBuf,
    #[allow(dead_code)]
    session: Arc<Mutex<Session>>,
    #[allow(dead_code)]
    options: RhaiScriptRunOptions,
    cache: Mutex<HashMap<PathBuf, Shared<Module>>>,
}

impl NativeImportResolver {
    pub fn new(
        main_script: &Path,
        session: Arc<Mutex<Session>>,
        options: RhaiScriptRunOptions,
    ) -> Self {
        let main_script_dir = main_script
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));

        Self {
            main_script_dir,
            session,
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
        ast.set_source(file_path.to_string_lossy().as_ref());

        let module = Module::eval_ast_as_new(Scope::new(), &ast, engine)
            .map_err(|err| Box::new(EvalAltResult::ErrorInModule(path.to_string(), err, pos)))?;

        return Ok(Shared::new(module));
    }
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
                _ => Err(Box::new(EvalAltResult::ErrorRuntime(
                    format!(
                        "import unsupported for extension '{ext}' (only .rhai is supported; \
                         importing a request by path/method is not yet implemented): {}",
                        resolved.display()
                    )
                    .into(),
                    pos,
                ))),
            }
        });
    }
}
