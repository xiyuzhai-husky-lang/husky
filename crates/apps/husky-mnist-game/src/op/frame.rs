use super::*;
use crate::trace::{
    input::Input, optimal_transport::OptimalTransport,
    optimal_transport_average::OptimalTransportAverage, skeleton::MnistSkeleton,
};

pub struct MnistOpFrame {
    skeleton: MnistSkeleton,
    skeleton_visual: Visual,
    optimal_transport: OptimalTransport,
    optimal_transport_visual: Visual,
    optimal_transport_average: OptimalTransportAverage,
    optimal_transport_average_visual: Visual,
}

impl MnistOpFrame {
    pub fn new(input: &Input, visual_synchrotron: &mut VisualSynchrotron) -> Self {
        let skeleton = MnistSkeleton::one();
        let skeleton_visual = skeleton.visualize(visual_synchrotron);
        let optimal_transport = OptimalTransport::new(input, &skeleton);
        let optimal_transport_visual = optimal_transport.visualize(visual_synchrotron);
        let optimal_transport_average = OptimalTransportAverage::new(&optimal_transport);
        let optimal_transport_average_visual =
            optimal_transport_average.visualize(visual_synchrotron);
        Self {
            skeleton,
            skeleton_visual,
            optimal_transport,
            optimal_transport_visual,
            optimal_transport_average,
            optimal_transport_average_visual,
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

    pub(crate) fn optimal_transport_average_visual(&self) -> Visual {
        self.optimal_transport_average_visual
    }
}
