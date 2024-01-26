use self::frame::{MnistFrame, MnistFramesToBe};
use super::*;

// stores information at a particular optimization step
pub struct MnistOpSnap {
    frame: MnistFrame,
    frames_to_be: MnistFramesToBe,
}

impl MnistOpSnap {
    pub(crate) fn new_ad_hoc(t: i32, visual_synchrotron: &mut VisualSynchrotron) -> MnistOpSnap {
        MnistOpSnap {
            frame: MnistFrame::new(visual_synchrotron),
            frames_to_be: MnistFramesToBe {},
        }
    }

    pub(crate) fn frame(&self) -> &MnistFrame {
        &self.frame
    }
}
