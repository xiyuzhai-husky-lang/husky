use crate::{trace::TraceSelection, MnistApp};

pub struct MnistChannels {
    channels: Vec<MnistChannel>,
    current: usize,
}

impl MnistChannels {
    pub fn new() -> Self {
        MnistChannels {
            channels: vec![
                MnistChannel::new(),
                MnistChannel::new(),
                MnistChannel::new(),
                MnistChannel::new(),
            ],
            current: 0,
        }
    }
}

impl std::ops::Deref for MnistChannels {
    type Target = [MnistChannel];

    fn deref(&self) -> &Self::Target {
        &self.channels
    }
}

pub struct MnistChannel {
    trace_selection: TraceSelection,
}
/// # constructors
impl MnistChannel {
    pub(crate) fn new() -> Self {
        Self {
            trace_selection: TraceSelection::default(),
        }
    }

    pub fn trace_selection_mut(&mut self) -> &mut TraceSelection {
        &mut self.trace_selection
    }
}

/// # ui
impl MnistChannel {
    pub(crate) fn ui(&self, ui: &mut egui::Ui) {
        ui.label("channel");
    }
}

impl MnistApp {
    pub fn current_channel_mut(&mut self) -> &mut MnistChannel {
        &mut self.channels.channels[self.channels.current]
    }
}
