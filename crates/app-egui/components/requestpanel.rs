use std::f32;

use strum::IntoEnumIterator;

use crate::components::Component;

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
    Trace,
}

pub struct RequestPanel {
    name: String,
    method: Method,
    request_tab: RequestTab,
}

impl RequestPanel {
    pub fn new() -> Self {
        return RequestPanel {
            name: String::new(),
            method: Method::default(),
            request_tab: RequestTab::default(),
        };
    }
}

impl Component for RequestPanel {
    type Response = egui::InnerResponse<()>;

    fn show(
        &mut self,
        ui: &mut egui::Ui,
        _state: &mut crate::state::ApplicationState,
        _runtime: &mut crate::runtime::RuntimeData,
    ) -> Self::Response {
        return ui.vertical(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("method")
                    .selected_text(format!("{}", self.method))
                    .show_ui(ui, |ui| {
                        for method in Method::iter() {
                            ui.selectable_value(
                                &mut self.method,
                                method.clone(),
                                method.to_string(),
                            );
                        }
                    });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("Send").clicked() {
                        println!("Sending {}", self.name);
                    };

                    ui.add(
                        egui::TextEdit::singleline(&mut self.name)
                            .hint_text("Enter url")
                            .desired_width(f32::INFINITY),
                    );
                });
            });

            ui.horizontal(|ui| {
                // request panel
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        for tab in RequestTab::iter() {
                            ui.selectable_value(
                                &mut self.request_tab,
                                tab.clone(),
                                tab.to_string(),
                            );
                        }
                    });
                    ui.label("The config will show here");
                    ui.label("The config will show here");
                });

                ui.vertical(|ui| {
                    ui.label("The response will show here");
                    ui.label("The response will show here");
                })
            });
        });
    }
}
