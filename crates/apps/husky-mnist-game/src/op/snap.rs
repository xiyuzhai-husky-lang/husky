use self::frame::{MnistFrame, MnistFramesToBe};
use super::*;

// stores information at a particular optimization step
pub struct MnistOpSnap {
    frame: MnistFrame,
    frames_to_be: MnistFramesToBe,
}

impl MnistOpSnap {
    pub(crate) fn new_ad_hoc(t: i32) -> MnistOpSnap {
        MnistOpSnap {
            frame: MnistFrame {},
            frames_to_be: MnistFramesToBe {},
        }
    }
}
