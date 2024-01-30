pub mod channel;
pub mod control;
pub mod ui_cache;

use self::{channel::MnistChannels, control::MnistControl};
use super::*;
use crate::ui_cache::MnistUiCache;

pub(crate) struct MnistApp {
    // backend
    pub(crate) db: MnistDb,
    // frontend
    pub(crate) control: MnistControl,
    pub(crate) channels: MnistChannels,
    pub(crate) ui_cache: MnistUiCache,
    // synchrotron
    pub(crate) visual_synchrotron: VisualSynchrotron,
}

/// # getters
impl MnistApp {
    pub(crate) fn control_mut(&mut self) -> &mut MnistControl {
        &mut self.control
    }
}

impl Default for MnistApp {
    fn default() -> Self {
        let mut visual_synchrotron = Default::default();
        let db = MnistDb::new(&mut visual_synchrotron);
        let control = MnistControl::new(&db);
        let channels = MnistChannels::new();
        let ui_cache = Default::default();
        Self {
            db,
            channels,
            control,
            ui_cache,
            visual_synchrotron,
        }
    }
}
