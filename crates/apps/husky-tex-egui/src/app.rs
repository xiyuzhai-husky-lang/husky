use egui::{CentralPanel, Color32, Galley, Label, Widget};
use std::sync::Arc;

#[derive(Default)]
pub(crate) struct TexEguiApp {}

impl eframe::App for TexEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| paint_text(ui.painter()));
    }
}

fn paint_text(painter: &egui::Painter) {
    let galley = Galley {
        job: todo!(),
        rows: todo!(),
        elided: todo!(),
        rect: todo!(),
        mesh_bounds: todo!(),
        num_vertices: todo!(),
        num_indices: todo!(),
        pixels_per_point: todo!(),
    };
    let galley = Arc::new(galley);
    painter.galley(Default::default(), galley, Color32::LIGHT_RED);
}
