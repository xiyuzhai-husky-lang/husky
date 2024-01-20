use crate::*;

pub trait Visualize {
    fn visualize(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual;
}

impl<T> Visualize for &T
where
    T: Visualize,
{
    fn visualize(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        T::visualize(self, visual_synchrotron)
    }
}

pub struct __VisualizeTest<T>(pub T);

impl<T> __VisualizeTest<T>
where
    T: Visualize,
{
    pub fn visualize(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        self.0.visualize(visual_synchrotron)
    }
}

impl<T> Visualize for __VisualizeTest<T> {
    fn visualize(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        __Visual::Void
    }
}
