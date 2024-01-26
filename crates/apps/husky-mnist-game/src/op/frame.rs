use super::*;
use crate::values::skeleton::Skeleton;

pub struct MnistFrame {
    skeleton: Skeleton,
    skeleton_visual: Visual,
}

impl MnistFrame {
    pub fn new(visual_synchrotron: &mut VisualSynchrotron) -> Self {
        let skeleton = Skeleton {};
        let skeleton_visual = skeleton.visualize(visual_synchrotron);
        Self {
            skeleton,
            skeleton_visual,
        }
    }
}

pub struct MnistFramesToBe {}
