use super::*;
use crate::values::skeleton::MnistSkeleton;

pub struct MnistFrame {
    skeleton: MnistSkeleton,
    skeleton_visual: Visual,
}

impl MnistFrame {
    pub fn new(visual_synchrotron: &mut VisualSynchrotron) -> Self {
        let skeleton = MnistSkeleton::one();
        let skeleton_visual = skeleton.visualize(visual_synchrotron);
        Self {
            skeleton,
            skeleton_visual,
        }
    }
}

/// # getters
impl MnistFrame {
    pub(crate) fn skeleton_visual(&self) -> Visual {
        self.skeleton_visual
    }
}

pub struct MnistFramesToBe {}
