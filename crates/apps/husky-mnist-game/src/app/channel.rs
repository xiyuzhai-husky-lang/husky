use crate::{
    trace::{Trace, TraceSelection},
    MnistApp,
};

#[derive(Debug)]
pub struct MnistChannels {
    channels: Vec<MnistChannel>,
    current: usize,
}

impl MnistChannels {
    pub fn new() -> Self {
        use crate::trace::Trace::*;
        macro_rules! channels {
            ($($args: expr),*) => {
                vec![$(MnistChannel::new($args)),*]
            };
            ($($args: expr,)*) => {
                vec![$(MnistChannel::new($args)),*]
            }
        }
        MnistChannels {
            channels: channels![
                [Input],
                [Input, Skeleton],
                [Input, Skeleton, OptimalTransport],
                [Input, Skeleton, OptimalTransportAverage],
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

#[derive(Debug)]
pub struct MnistChannel {
    trace_selection: TraceSelection,
}
/// # constructors
impl MnistChannel {
    pub(crate) fn new(traces: impl IntoIterator<Item = Trace>) -> Self {
        Self {
            trace_selection: TraceSelection::new(traces),
        }
    }

    pub fn trace_selection(&self) -> &TraceSelection {
        &self.trace_selection
    }

    pub fn trace_selection_mut(&mut self) -> &mut TraceSelection {
        &mut self.trace_selection
    }
}

impl MnistApp {
    pub fn current_channel_mut(&mut self) -> &mut MnistChannel {
        &mut self.channels.channels[self.channels.current]
    }
}
