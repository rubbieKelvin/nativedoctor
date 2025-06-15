use std::slice::Iter;

use dioxus::{
    hooks::{use_context, use_context_provider},
    signals::{Readable, Signal, Writable, WritableVecExt},
};

#[derive(Clone, PartialEq, Debug)]
pub enum ToastTitle {
    Info(String),
    Error(String),
    Debug(String),
    Warning(String),
}

impl Into<String> for ToastTitle {
    fn into(self) -> String {
        return match self {
            ToastTitle::Debug(s) => s,
            ToastTitle::Error(s) => s,
            ToastTitle::Info(s) => s,
            ToastTitle::Warning(s) => s,
        };
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ToastCloseMethod {
    None,
    Button,
    Timeout(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastConfig {
    pub id: uuid::Uuid,
    pub title: ToastTitle,
    pub summary: Option<String>,
    pub close_method: ToastCloseMethod,
}

impl ToastConfig {
    pub fn new(title: ToastTitle, summary: Option<String>, close_method: ToastCloseMethod) -> Self {
        return ToastConfig {
            id: uuid::Uuid::new_v4(),
            title,
            summary,
            close_method,
        };
    }
}

#[derive(Clone, PartialEq)]
pub struct ToastState {
    toasts: Signal<Vec<ToastConfig>>,
}

impl ToastState {
    pub fn provide() -> ToastState {
        return use_context_provider(|| ToastState {
            toasts: Signal::new(vec![]),
        });
    }

    pub fn inject() -> ToastState {
        return use_context::<ToastState>();
    }

    pub fn items(&self) -> Vec<ToastConfig> {
        let toasts = self.toasts.read();
        return toasts.clone();
    }

    pub fn push(&mut self, config: ToastConfig) {
        self.toasts.with_mut(|toasts| {
            toasts.push(config);
        })
    }

    pub fn remove(&mut self, id: uuid::Uuid) {
        self.toasts.with_mut(|toasts| {
            toasts.retain(|config| config.id != id);
        })
    }
}
