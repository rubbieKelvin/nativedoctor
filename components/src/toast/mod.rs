use dioxus::prelude::*;
use dioxus::{
    hooks::{use_context, use_context_provider},
    signals::{Readable, Signal, Writable},
};
use dioxus_free_icons::{Icon, icons::ld_icons::LdX};

use crate::border::Border;
use crate::button::{Button, ButtonSizeVariant, ButtonStyleVariant};
use crate::label::{Label, LabelSizeVariant, LabelStyleVariant};
use crate::pane::{Pane, PaneStyleVariant};
use crate::traits::Variant;

#[derive(Clone, Copy, PartialEq, Debug, Default, strum::EnumIter, strum::Display)]
pub enum ToastVariant {
    #[default]
    Info,
    Error,
    Warning,
}

impl Variant for ToastVariant {
    fn classes(&self) -> &'static str {
        return match self {
            ToastVariant::Info => "",
            ToastVariant::Error => "!bg-[#3d2024]",
            ToastVariant::Warning => "!bg-[#5d5120]",
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
    pub title: String,
    pub variant: ToastVariant,
    pub close_method: ToastCloseMethod,
}

impl ToastConfig {
    pub fn new(title: String, variant: ToastVariant, close_method: ToastCloseMethod) -> Self {
        return ToastConfig {
            id: uuid::Uuid::new_v4(),
            title,
            variant: variant,
            close_method,
        };
    }
}

impl Into<ToastConfig> for String {
    fn into(self) -> ToastConfig {
        return ToastConfig {
            id: uuid::Uuid::new_v4(),
            title: self,
            variant: ToastVariant::Info,
            close_method: ToastCloseMethod::Button,
        };
    }
}

impl Into<ToastConfig> for &'static str {
    fn into(self) -> ToastConfig {
        return self.to_string().into();
    }
}

#[derive(Clone, PartialEq, Copy)]
pub struct ToastState {
    toasts: Signal<Vec<ToastConfig>>,
}

impl ToastState {
    fn provide() -> ToastState {
        return use_context_provider(|| ToastState {
            toasts: Signal::new(vec![]),
        });
    }

    fn inject() -> ToastState {
        return use_context::<ToastState>();
    }

    fn items(&self) -> Vec<ToastConfig> {
        let toasts = self.toasts.read();
        return toasts.clone();
    }

    pub fn push(&mut self, config: ToastConfig) {
        tracing::debug!("Pushed toast: {:?}", &config);
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

pub fn use_toast() -> ToastState {
    return ToastState::inject();
}

#[component]
pub fn ToastProvider(children: Element) -> Element {
    let toast_state = ToastState::provide();

    return rsx! {
        div { class: "w-96 p-2 fixed right-0 bottom-0 z-50",
            div {
                class: "flex flex-col gap-2",
                for item in toast_state.items() {
                    ToastItem { key: item.id, toast: item }
                }
            }
        }
        {children}
    };
}

#[component]
fn ToastItem(toast: ToastConfig) -> Element {
    let mut toast_state = ToastState::inject();
    let title: String = toast.title.into();
    let class = toast.variant.classes();

    return rsx! {
        Pane {
            class: "p-1 flex items-center rounded-md {class}",
            style: PaneStyleVariant::Lighter,
            border: Border::all(),
            Label {
                class: "flex-grow",
                style: LabelStyleVariant::Mild,
                size: LabelSizeVariant::Small,
                "{title}"
            }

            Button {
                style: ButtonStyleVariant::Ghost,
                size: ButtonSizeVariant::Icon,
                onclick: move |_| {
                    toast_state.remove(toast.id);
                },
                Icon{
                    width: 16,
                    height: 16,
                    icon: LdX
                }
            }
        }
    };
}
