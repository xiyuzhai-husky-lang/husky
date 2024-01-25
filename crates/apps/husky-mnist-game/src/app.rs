use egui::{SidePanel, TopBottomPanel, Widget};

use self::components::channel::MnistChannel;
use super::*;
use crate::components::{channel::MnistChannels, control::MnistControl};

pub(crate) struct MnistApp {
    // db
    db: MnistDb,
    // components
    control: MnistControl,
    channels: MnistChannels,
}

/// # getters
impl MnistApp {
    pub(crate) fn channels(&self) -> &[MnistChannel] {
        self.channels.as_ref()
    }

    pub(crate) fn control_mut(&mut self) -> &mut MnistControl {
        &mut self.control
    }
}

impl Default for MnistApp {
    fn default() -> Self {
        let db = MnistDb::default();
        let control = MnistControl::new(&db);
        let channels = vec![
            MnistChannel::new(),
            MnistChannel::new(),
            MnistChannel::new(),
            MnistChannel::new(),
        ];
        Self {
            db,
            channels,
            control,
        }
    }
}

impl eframe::App for MnistApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.layout(ctx);
    }
}
