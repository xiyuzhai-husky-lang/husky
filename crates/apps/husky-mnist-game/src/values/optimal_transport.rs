use self::{input::Input, skeleton::MnistSkeleton};
use super::*;

pub struct OptimalTransport {}

impl OptimalTransport {
    pub(crate) fn new(input: &Input, skeleton: &MnistSkeleton) -> Self {
        Self {}
    }
}

impl Visualize for OptimalTransport {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        // ad hoc
        Visual::Void
    }
}
