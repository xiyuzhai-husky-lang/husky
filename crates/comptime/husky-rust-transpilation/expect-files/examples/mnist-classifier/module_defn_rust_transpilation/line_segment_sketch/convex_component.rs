use super::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
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
    fn visualize(&self, __visual_synchrotron: &mut __VisualSynchrotron) -> husky_core::visual::Visual {
        self.line_segments.visualize(__visual_synchrotron)
    }
}