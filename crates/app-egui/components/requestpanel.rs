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
}

impl RequestDashboard {
    pub fn new() -> Self {
        return RequestDashboard {
            name: String::new(),
            method: Method::default(),
            request_tab: RequestTab::default(),
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
                .width(90.0)
                .show_ui(ui, |ui| {
                    for method in Method::iter() {
                        ui.selectable_value(&mut self.method, method.clone(), method.to_string());
                    }
                });

            let url_response = ui.add(
                egui::TextEdit::singleline(&mut self.name)
                    .hint_text("Enter url")
                    .min_size((90.0, 20.0).into())
                    .desired_width(ui.available_width() - 105.0),
            );

            let click_button = egui::Button::new("Send").min_size((90.0, 20.0).into());
            
            if ui.add(click_button).clicked()
                || (url_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                println!("Sending {}", self.name);
            };
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
        let visuals = ui.style().visuals.clone();

        return ui.vertical(|ui| {
            // Select method, show url and show button
            self.show_top_input(ui, state, runtime);
            ui.add_space(5f32);

            // Request response panels
            ui.with_layout(Layout::left_to_right(egui::Align::LEFT), |ui| {
                // request panel
                let spacing = 5f32;
                let request_panel_frame =
                    egui::Frame::default().stroke(visuals.widgets.noninteractive.bg_stroke);

                request_panel_frame.show(ui, |ui| {
                    ui.add_space(spacing);
                    ui.vertical(|ui| {
                        // panel tab
                        ui.add_space(spacing);
                        ui.horizontal(|ui| {
                            for tab in RequestTab::iter() {
                                ui.selectable_value(
                                    &mut self.request_tab,
                                    tab.clone(),
                                    tab.to_string(),
                                );
                            }
                        });

                        // show request panel input
                        RequestInputPanel {
                            tab: self.request_tab.clone(),
                        }
                        .show(ui, state, runtime);
                        ui.add_space(spacing);
                    });
                    ui.add_space(spacing);
                });

                ui.vertical(|ui| {
                    ui.label("The response will show here");
                    ui.label("The response will show here");
                })
            });
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
