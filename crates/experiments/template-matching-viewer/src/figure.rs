use crate::*;

#[derive(Debug, Default)]
pub struct Figure {}

impl TemplateMatchingViewerApp {
    pub(crate) fn render_figure_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| self.render_figure_bar(ctx, ui));
    }

    fn figure_frame(&self) -> egui::Frame {
        egui::Frame {
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Default::default(),
            shadow: Default::default(),
            fill: egui::Color32::WHITE,
            stroke: Default::default(),
        }
    }

    fn render_figure_bar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {}
}
