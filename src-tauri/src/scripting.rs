//! Rhai scripting support. Scripts can be run via `engine.run(&script)` or
//! `engine.eval::<T>(&script)`; errors use [`rhai::EvalAltResult`].

use rhai::Engine;

/// Creates a new Rhai engine for script evaluation.
#[allow(unused)]
pub fn new_engine() -> Engine {
    Engine::new()
}
