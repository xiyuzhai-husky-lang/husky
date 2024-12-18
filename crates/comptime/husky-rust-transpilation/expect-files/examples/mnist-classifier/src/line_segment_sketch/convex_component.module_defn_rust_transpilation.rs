use super::*;
#[rustfmt::skip]
#[allow(non_upper_case_globals)]
pub static mut __ConvexComponent__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct ConvexComponent {
    pub line_segment_sketch: Leash<crate::line_segment_sketch::LineSegmentSketch>,
    pub line_segments: CyclicSliceLeashed<crate::line_segment_sketch::LineSegmentStroke>,
}

impl ConvexComponent {
    pub fn __constructor(line_segment_sketch: Leash<crate::line_segment_sketch::LineSegmentSketch>, line_segments: CyclicSliceLeashed<crate::line_segment_sketch::LineSegmentStroke>) -> Self {
        Self{
            line_segment_sketch,
            line_segments,
        }
    }
}

#[rustfmt::skip]
impl Visualize for crate::line_segment_sketch::convex_component::ConvexComponent {
    fn visualize(&self, __visual_synchrotron: &mut __VisualSynchrotron) -> Visual {
        self.line_segments.deleash().visualize(__visual_synchrotron)
    }
}
#[rustfmt::skip]
#[allow(non_upper_case_globals)]
pub static mut __Visualize__for__ConvexComponent__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

