use self::values::{input::Input, optimal_transport::OptimalTransport};
use super::*;
use crate::values::skeleton::MnistSkeleton;

pub struct MnistOpFrame {
    skeleton: MnistSkeleton,
    skeleton_visual: Visual,
    optimal_transport: OptimalTransport,
    optimal_transport_visual: Visual,
}

impl MnistOpFrame {
    pub fn new(input: &Input, visual_synchrotron: &mut VisualSynchrotron) -> Self {
        let skeleton = MnistSkeleton::one();
        let skeleton_visual = skeleton.visualize(visual_synchrotron);
        let optimal_transport = OptimalTransport::new_ad_hoc(input, &skeleton);
        let optimal_transport_visual = optimal_transport.visualize(visual_synchrotron);
        Self {
            skeleton,
            skeleton_visual,
            optimal_transport,
            optimal_transport_visual,
        }
    }
}

/// # getters
impl MnistOpFrame {
    pub(crate) fn skeleton_visual(&self) -> Visual {
        self.skeleton_visual
    }

    pub(crate) fn optimal_transport_visual(&self) -> Visual {
        self.optimal_transport_visual
    }
}

pub struct MnistFramesToBe {}
