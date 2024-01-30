use self::frame::MnistOpFrame;
use super::*;
use crate::trace::input::Input;
use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpTime(ShiftedU32);

impl OpTime {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}

pub struct MnistOpHistory {
    op_frames: Vec<MnistOpFrame>,
}

impl MnistOpHistory {
    pub fn new(input: &Input, visual_synchrotron: &mut VisualSynchrotron) -> Self {
        let op_frames = (0..10)
            .into_iter()
            .map(|_t| MnistOpFrame::new(input, visual_synchrotron))
            .collect();
        Self { op_frames }
    }

    pub fn op_frames(&self) -> &[MnistOpFrame] {
        self.op_frames.as_ref()
    }
}
