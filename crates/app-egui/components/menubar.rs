use crate::{components::Component, shortcuts};

#[derive(Default)]
pub struct Menubar;

impl Component for Menubar {
    // fn show(&self, ui: &mut egui::Ui) -> egui::InnerResponse<()> {
    // }
    type Response = egui::InnerResponse<()>;
    fn show(
        &mut self,
        ctx: &egui::Context,
        _state: &mut crate::state::ApplicationState,
        _runtime: &mut crate::runtime::RuntimeData,
    ) -> Self::Response {
        return egui::TopBottomPanel::top("menu_bar")
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin {
                        left: 80,
                        top: 8,
                        bottom: 8,
                        ..Default::default()
                    })
                    .fill(ctx.style().visuals.panel_fill),
            )
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    // handle shortcuts
                    if ui.input_mut(|i| i.consume_shortcut(&shortcuts::FILE_NEW)) {
                        println!("Clicked button from shorcut");
                    }

                    ui.menu_button("File", |ui| {
                        if ui
                            .add(
                                egui::Button::new("New")
                                    .shortcut_text(ui.ctx().format_shortcut(&shortcuts::FILE_NEW)),
                            )
                            .clicked()
                        {
                            println!("Clicked button");
                        }
                    });
                });
            });
    }
}
