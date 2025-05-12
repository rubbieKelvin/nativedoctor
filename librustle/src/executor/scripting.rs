use std::rc::Rc;

use anyhow::{Context, Result};
use deno_core::{error::CoreError, extension, v8, FsModuleLoader, JsRuntime, RuntimeOptions};
use tracing::info;

use super::runner::Runner;

pub struct JavascriptExec {
    runtime: JsRuntime,
}

const pre_script: &'static str = "
    const { core } = Deno;
    const log = core.ops.op_log;
    const console = undefined;
    function assert(condition, message){
        if (!condition){
            let err = new Error(message || 'Assertation failed');
            err.name = 'AssertationError';
            throw err;
        }
    }
";

impl JavascriptExec {
    pub fn new() -> Result<Self> {
        extension!(rustle, ops = [op_log]);

        let runtime = JsRuntime::new(RuntimeOptions {
            // module_loader: None, // we wont be loading js modules from here, so we can ignore this
            module_loader: Some(Rc::new(FsModuleLoader)),
            extensions: vec![rustle::init()],
            ..Default::default()
        });

        return Ok(JavascriptExec { runtime });
    }

    // execute the js snippet within the deno runtime
    pub fn run(&mut self, script: &str, rustle_exec: Option<&mut Runner>) -> Result<()> {
        let runtime = &mut self.runtime;

        // enter the v8 scope
        let context = runtime.main_context();
        let scope = &mut runtime.handle_scope();
        let global = context.open(scope);

        if let Some(rustle) = rustle_exec {
            // TODO: Set global values/variables
            let env_v8_obj = v8::Object::new(scope);
        }

        let mut js = String::from(pre_script);
        js.push_str(script);

        let script = v8::String::new(scope, &js).context("Cannot create v8 string")?;
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

// Op definition for assert to be used in js runtims
#[deno_core::op2(fast)]
fn op_log(#[string] message: String) -> std::result::Result<(), CoreError> {
    println!("[JS]: {message}");
    return Ok(());
}

#[derive(Debug)]
enum ConsoleOutput {
    Info { message: String },
    Debug { message: String },
    Error { message: String },
    Warning { message: String },
}

pub fn run_js(output: &mut Vec<ConsoleOutput>) -> Result<()> {
    extension!(rustle, ops = [op_logger_info]);

    // Op definition for assert to be used in js runtims
    #[deno_core::op2(fast)]
    fn op_logger_info(#[string] message: String) -> std::result::Result<(), CoreError> {
        info!("JS: {message}");
        output;
        return Ok(());
    }

    let runtime = JsRuntime::new(RuntimeOptions {
        // module_loader: None, // we wont be loading js modules from here, so we can ignore this
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![rustle::init()],
        ..Default::default()
    });
    return Ok(());
}
