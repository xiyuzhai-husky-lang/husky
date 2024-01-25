use crate::MnistApp;

pub type MnistChannels = Vec<MnistChannel>;

pub struct MnistChannel {
    trace_selection: TraceSelection,
}
/// # constructors
impl MnistChannel {
    pub(crate) fn new() -> Self {
        Self {
            trace_selection: TraceSelection {},
        }
    }
}

/// # ui
impl MnistChannel {
    pub(crate) fn ui(&self, ui: &mut egui::Ui) {
        ui.label("channel");
    }
}

pub struct TraceSelection {}
