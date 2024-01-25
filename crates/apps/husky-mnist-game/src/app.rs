pub mod channel;
pub mod control;
pub mod ui_cache;

use self::{
    channel::{MnistChannel, MnistChannels},
    control::MnistControl,
};
use super::*;
use crate::ui_cache::MnistUiCache;
use egui::{SidePanel, TopBottomPanel, Widget};

pub(crate) struct MnistApp {
    // backend
    pub(crate) db: MnistDb,
    // frontend
    pub(crate) control: MnistControl,
    pub(crate) channels: MnistChannels,
    pub(crate) ui_cache: MnistUiCache,
}

/// # getters
impl MnistApp {
    pub(crate) fn channels(&self) -> &MnistChannels {
        &self.channels
    }

    pub(crate) fn control_mut(&mut self) -> &mut MnistControl {
        &mut self.control
    }
}

impl Default for MnistApp {
    fn default() -> Self {
        let db = MnistDb::default();
        let control = MnistControl::new(&db);
        let channels = MnistChannels::new();
        let ui_cache = Default::default();
        Self {
            db,
            channels,
            control,
            ui_cache,
        }
    }
}
