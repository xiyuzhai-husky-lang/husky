use crate::*;


pub(crate) fn is_convex<'eval>(line_segment_sketch: &crate::line_segment_sketch::LineSegmentSketch<'eval>, index: i32) -> bool {
    let L = line_segment_sketch.line_segments.ilen();
    let current_displacement = line_segment_sketch.line_segments[(index % L) as usize].displacement();
    let previous_displacement = line_segment_sketch.line_segments[((index - 1) % L) as usize].displacement();
    let is_rotation_counterclockwise_result = previous_displacement.rotation_direction_to(&current_displacement);
    if is_rotation_counterclockwise_result == 0 {
        let mut previous_raw_cross = -999999f32;
        let previous_interval = line_segment_sketch.line_segments[((index - 1) % L) as usize].points;
        for i1 in previous_interval.start..previous_interval.end {
            let displacement = line_segment_sketch.contour.displacement(previous_interval.start, i1);
            previous_raw_cross = previous_raw_cross.max(current_displacement.cross(&displacement));
        }
        let mut current_raw_cross = -999999f32;
        let current_interval = line_segment_sketch.line_segments[(index % L) as usize].points;
        for i2 in current_interval.start..current_interval.end {
            let displacement = line_segment_sketch.contour.displacement(previous_interval.start, i2);
            current_raw_cross = current_raw_cross.max(current_displacement.cross(&displacement));
        }
        return current_raw_cross < previous_raw_cross
    } else {
        return is_rotation_counterclockwise_result > 0
    }
}
