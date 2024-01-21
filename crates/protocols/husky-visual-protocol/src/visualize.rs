mod primitive;

use self::{synchrotron::VisualSynchrotron, visual::Visual};
use super::*;

pub trait Visualize {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual;
}

impl<T> Visualize for &T
where
    T: Visualize,
{
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        T::visualize(self, visual_synchrotron)
    }
}

impl<T> Visualize for Vec<T>
where
    T: Visualize,
{
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        let elements = self
            .iter()
            .map(|element| element.visualize(visual_synchrotron))
            .collect();
        Visual::new_group_visual(elements, visual_synchrotron)
    }
}

pub struct VisualizeTest<T>(pub T);

impl<T> VisualizeTest<T>
where
    T: Visualize,
{
    pub fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        self.0.visualize(visual_synchrotron)
    }
}

impl<T> Visualize for VisualizeTest<T> {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        Visual::Void
    }
}
