use egui::Layout;
use strum::IntoEnumIterator;

use crate::{components::Component, runtime, state};

#[derive(strum::Display, strum::EnumIter, Default, Clone, PartialEq)]
enum RequestTab {
    Params,
    Header,
    Auth,
    #[default]
    Body,
    Doc,
    Script,
}

#[derive(strum::Display, strum::EnumIter, Default, Clone, PartialEq)]
enum ResponseTab {
    #[default]
    Body,
    Headers,
    Cookies,
    ScriptOutput,
}

#[derive(strum::Display, strum::EnumIter, Default, Clone, PartialEq)]
enum Method {
    #[default]
    Get,
    Delete,
    Post,
    Patch,
    Put,
    Head,
    Option,
}

pub struct RequestDashboard {
    name: String,
    method: Method,
    request_tab: RequestTab,
    response_tab: ResponseTab,
}

impl RequestDashboard {
    pub fn new() -> Self {
        return RequestDashboard {
            name: String::new(),
            method: Method::default(),
            request_tab: RequestTab::default(),
            response_tab: ResponseTab::default(),
        };
    }
}

impl RequestDashboard {
    fn show_top_input(
        &mut self,
        ui: &mut egui::Ui,
        _state: &mut state::ApplicationState,
        _runtime: &mut runtime::RuntimeData,
    ) -> egui::InnerResponse<()> {
        return ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("method")
                .selected_text(format!("{}", self.method))
                .width(80.0)
                .show_ui(ui, |ui| {
                    for method in Method::iter() {
                        ui.selectable_value(&mut self.method, method.clone(), method.to_string());
                    }
                });

            let url_response = ui.add(
                egui::TextEdit::singleline(&mut self.name)
                    .interactive(true)
                    .id_salt("url_input")
                    .hint_text("Enter url")
                    .desired_width(ui.available_width() - 105.0),
            );

            if url_response.clicked() {
                url_response.request_focus();
            }

            let click_button = egui::Button::new("Send").min_size((60., 0.).into());

            if ui.add(click_button).clicked()
                || (url_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                println!("Sending {}", self.name);
            };
        });
    }

    fn show_request_input_panel(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut state::ApplicationState,
        runtime: &mut runtime::RuntimeData,
    ) -> egui::InnerResponse<()> {
        return ui.vertical(|ui| {
            ui.horizontal(|ui| {
                for tab in RequestTab::iter() {
                    ui.selectable_value(&mut self.request_tab, tab.clone(), tab.to_string());
                }
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.label(self.request_tab.to_string());
            });
        });
    }

    fn show_response_output_panel(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut state::ApplicationState,
        runtime: &mut runtime::RuntimeData,
    ) -> egui::InnerResponse<()> {
        return ui.vertical(|ui| {
            ui.horizontal(|ui| {
                for tab in ResponseTab::iter() {
                    ui.selectable_value(&mut self.response_tab, tab.clone(), tab.to_string());
                }
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.label(self.response_tab.to_string());
            });
        });
    }
}

impl Component for RequestDashboard {
    type Response = egui::InnerResponse<()>;
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut state::ApplicationState,
        runtime: &mut runtime::RuntimeData,
    ) -> Self::Response {
        return ui.vertical(|ui| {
            self.show_top_input(ui, state, runtime);

            ui.spacing();
            self.show_request_input_panel(ui, state, runtime);

            ui.spacing();
            self.show_response_output_panel(ui, state, runtime);
        });
    }
}

struct RequestInputPanel {
    tab: RequestTab,
}

impl Component for RequestInputPanel {
    type Response = egui::Response;

    fn show(
        &mut self,
        ui: &mut egui::Ui,
        _state: &mut crate::state::ApplicationState,
        _runtime: &mut crate::runtime::RuntimeData,
    ) -> Self::Response {
        return match self.tab {
            RequestTab::Auth => ui.label("Auth"),
            RequestTab::Body => ui.label("Body"),
            RequestTab::Doc => ui.label("Doc"),
            RequestTab::Header => ui.label("Header"),
            RequestTab::Params => ui.label("Params"),
            RequestTab::Script => ui.label("Script"),
        };
    }
}
