use super::*;

pub struct Skeleton {}

impl Visualize for Skeleton {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        Visual::Void
    }
}
