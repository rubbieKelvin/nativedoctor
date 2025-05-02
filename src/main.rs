// struct Application {
//     count: i32,
// }

// impl Application {
//     fn new() -> Self {
//         return Application { count: 0 };
//     }
// }

// impl eframe::App for Application {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             if ui.button("-").clicked() {
//                 self.count -= 1;
//             }
//             ui.label(self.count.to_string());
//             if ui.button("+").clicked() {
//                 self.count += 1;
//             }
//         });
//     }
// }

// fn main() -> Result<(), eframe::Error> {
//     let application_name = "dotapi";
//     let options = eframe::NativeOptions::default();
//     return eframe::run_native(
//         application_name,
//         options,
//         Box::new(|_context| Ok(Box::new(Application::new()))),
//     );
// }

fn main() -> {

}
