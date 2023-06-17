use crate::*;
use egui::{Color32, ColorImage};

pub struct Figure {
    retained_image: egui_extras::image::RetainedImage,
    texture: Option<egui::TextureHandle>,
}

impl Default for Figure {
    fn default() -> Self {
        Self {
            retained_image: egui_extras::image::RetainedImage::from_color_image(
                "default_image",
                ColorImage::new([400, 400], Color32::RED),
            ),
            texture: Default::default(),
        }
    }
}

impl App {
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

    fn render_figure_bar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.figure.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "my-image",
                color_image_example(self.control.r(), self.control.g(), self.control.b()),
                Default::default(),
            )
        });

        // Show the image:
        // ui.add(egui::Image::new(texture, texture.size_vec2()));

        // Shorter version:
        ui.image(texture, texture.size_vec2());
    }
}

fn color_image_example(r: u8, g: u8, b: u8) -> ColorImage {
    ColorImage::new([400, 400], Color32::from_rgb(r, g, b))
}
