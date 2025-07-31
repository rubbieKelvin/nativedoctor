use eframe::egui::{self, StrokeKind};
use egui::{Color32, Pos2, Stroke};

struct CanvasApp {
    points: Vec<Pos2>, // Store points for drawing
}

impl CanvasApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { points: Vec::new() }
    }
}

impl eframe::App for CanvasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Define the canvas area
            let canvas_size = egui::vec2(400.0, 300.0);
            let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::drag());

            // Get the canvas rectangle
            let canvas_rect = response.rect;

            // Draw a white background for the canvas
            painter.rect_filled(canvas_rect, 0.0, Color32::WHITE);

            // Handle mouse input to draw points
            if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    self.points.push(pos - canvas_rect.min.to_vec2()); // Convert to Vec2 and adjust
                }
            }

            // Draw all stored points
            for &point in &self.points {
                let abs_point = point + canvas_rect.min.to_vec2();
                painter.circle_filled(abs_point, 2.0, Color32::BLACK);
            }

            // Optional: Draw a border around the canvas
            painter.rect_stroke(
                canvas_rect,
                0.0,                              // Rounding
                Stroke::new(1.0, Color32::BLACK), // Stroke width and color
                StrokeKind::Middle,               // Add StrokeKind
            );
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Egui Canvas",
        options,
        Box::new(|cc| Ok(Box::new(CanvasApp::new(cc)))),
    )
}
