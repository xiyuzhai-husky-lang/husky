use crate::{InputId, MnistDb};
use egui::Slider;

pub struct MnistControl {
    input_id: InputId,
    frame_idx: usize,
    number_of_frames: usize,
}

impl MnistControl {
    pub(crate) fn new(engine: &MnistDb) -> MnistControl {
        let input_id = InputId::from_index(0);
        let number_of_frames = engine.frames(input_id).len();
        Self {
            input_id,
            frame_idx: 0,
            number_of_frames,
        }
    }
}

impl egui::Widget for &mut MnistControl {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("time");
        ui.style_mut().spacing.slider_width = ui.available_width() - 155.0;
        Slider::new(&mut self.frame_idx, 0..=(self.number_of_frames - 1)).ui(ui)
    }
}
