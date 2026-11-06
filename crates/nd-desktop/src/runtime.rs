//! A tokio runtime for networking. GPUI has its own executor, so requests run here and report
//! back over channels.

use gpui_kit::{App, Global};
use tokio::runtime::{Handle, Runtime};

pub struct NetRuntime(Runtime);

impl Global for NetRuntime {}

impl NetRuntime {
    pub fn handle(&self) -> Handle {
        return self.0.handle().clone();
    }
}

pub fn init(cx: &mut App) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .thread_name("nd-net")
        .build()
        .expect("tokio runtime");
    cx.set_global(NetRuntime(runtime));
}

pub fn handle(cx: &App) -> Handle {
    return cx.global::<NetRuntime>().handle();
}
