use crate::*;

#[derive(Debug, Default)]
pub struct Control {}

impl TemplateMatchingViewerApp {
    pub(crate) fn render_control_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("control_panel")
            .frame(self.control_frame())
            .show(ctx, |ui| self.render_control_bar_content(ctx, ui));
    }

    fn control_frame(&self) -> egui::Frame {
        egui::Frame {
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Default::default(),
            shadow: Default::default(),
            fill: egui::Color32::GRAY,
            stroke: Default::default(),
        }
    }

    fn render_control_bar_content(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {}
}
