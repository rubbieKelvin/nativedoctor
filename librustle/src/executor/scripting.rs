use std::borrow::Cow;

use anyhow::{Context, Ok, Result};
use deno_core::v8;
use deno_core::{Extension, JsRuntime, RuntimeOptions};

pub struct JavascriptRunner {
    runtime: JsRuntime,
}

impl JavascriptRunner {
    pub fn new() -> Result<Self> {
        let extensions = Extension {
            name: "rustle_post_request_script",
            ops: Cow::Borrowed(&[]),
            ..Default::default()
        };

        let runtime = JsRuntime::new(RuntimeOptions {
            module_loader: None, // we wont be loading js modules from here, so we can ignore this
            extensions: vec![extensions],
            ..Default::default()
        });

        return Ok(JavascriptRunner { runtime });
    }

    // execute the js snippet within the deno runtime
    pub fn run(&mut self, script: &str) -> Result<()> {
        let runtime = &mut self.runtime;

        // enter the v8 scope
        let context = runtime.main_context();
        let scope = &mut runtime.handle_scope();
        let global = context.open(scope);

        // TODO: Set global values/variables
        // ...

        let script = v8::String::new(scope, script).context("Cannot create v8 string")?;
        let scr_name =
            v8::String::new(scope, "<untitled_script>").context("Cannot create v8 string")?;
        let origin = v8::ScriptOrigin::new(
            scope,
            scr_name.into(),
            0,
            0,
            false,
            0,
            None,
            false,
            false,
            false,
            None,
        );

        let try_catch_scope = &mut v8::TryCatch::new(scope);
        let compiled_script = v8::Script::compile(try_catch_scope, script, Some(&origin))
            .context("Failed to compile script")?;

        let result = compiled_script.run(try_catch_scope);

        if result.is_none() {
            let exception = try_catch_scope.exception().unwrap();
            let err_string = exception.to_rust_string_lossy(try_catch_scope);
            anyhow::bail!("Script execution failed: {}", err_string);
        }

        return Ok(());
    }
}
