use crate::*;

#[derive(Default)]
pub struct Explorer {}

impl App {
    pub(crate) fn render_explorer_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("explorer_panel")
            .frame(self.explorer_frame())
            .show(ctx, |ui| self.render_explorer_bar(ctx, ui));
    }

    fn explorer_frame(&self) -> egui::Frame {
        egui::Frame {
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Default::default(),
            shadow: Default::default(),
            fill: egui::Color32::DARK_GRAY,
            stroke: Default::default(),
        }
    }

    fn render_explorer_bar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {}
}
