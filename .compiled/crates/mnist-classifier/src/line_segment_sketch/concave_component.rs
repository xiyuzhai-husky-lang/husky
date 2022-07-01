use crate::*;

pub struct ConcaveComponent<'eval> {
    pub(crate) line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
    pub(crate) line_segments:
        __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegment<'eval>>,
}

impl<'eval> ConcaveComponent<'eval> {}

pub(crate) fn find_concave_components<'eval>(
    line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
) -> Vec<crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>> {
    let mut concave_components =
        Vec::<crate::line_segment_sketch::concave_component::ConcaveComponent>::__call__();
    let L = line_segment_sketch.line_segments.ilen();
    let mut start = 0i32;
    let mut end = 1i32;
    while start > -L
        && !crate::line_segment_sketch::convexity::is_convex(&line_segment_sketch, start)
    {
        start -= 1;
    }

    let ccv_start = start;
    while start < ccv_start + L {
        while end <= start + L
            && !crate::line_segment_sketch::convexity::is_convex(&line_segment_sketch, end)
        {
            end += 1;
        }

        if end > start + 1i32 {
            concave_components.push(
                crate::line_segment_sketch::concave_component::ConcaveComponent::__call__(
                    line_segment_sketch,
                    line_segment_sketch.line_segments.cyclic_slice(start, end),
                ),
            );
        }
        start = end;
        end = start + 1i32;
    }

    return concave_components;
}
