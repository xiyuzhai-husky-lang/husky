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

#[macro_export]
macro_rules! LineSegment {
    ($start: expr, $end: expr, $visual_synchrotron: ident) => {{
        debug_assert_eq!($start.0, "start");
        let start = $start.1;
        debug_assert_eq!($end.0, "end");
        let end = $end.1;
        Visual::new_line_segment((start.x, start.y), (end.x, end.y), $visual_synchrotron)
    }};
}

#[macro_export]
macro_rules! Contour {
    ($points: expr, $visual_synchrotron: ident) => {{
        debug_assert_eq!($points.0, "points");
        let points = $points.1;
        Visual::new_contour(
            points.iter().map(|point| (point.x, point.y)),
            $visual_synchrotron,
        )
    }};
}
