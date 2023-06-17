use crate::*;

#[derive(Debug, Default)]
pub struct Control {
    r: u8,
    g: u8,
    b: u8,
}

impl Control {
    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

impl App {
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

    fn render_control_bar_content(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.control.r, 0..=120).text("r"));
        ui.add(egui::Slider::new(&mut self.control.g, 0..=120).text("g"));
        ui.add(egui::Slider::new(&mut self.control.b, 0..=120).text("b"));
    }
}
