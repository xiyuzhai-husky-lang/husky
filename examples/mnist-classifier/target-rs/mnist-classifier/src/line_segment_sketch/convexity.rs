use super::*;

#[allow(non_upper_case_globals)]
pub static mut __is_convex__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn is_convex(line_segment_sketch: &crate::line_segment_sketch::LineSegmentSketch, index: i32) -> bool {
    let L = line_segment_sketch.strokes.ilen();
    let current_displacement = line_segment_sketch.strokes[index.rem_euclid(L) as usize].displacement();
    let previous_displacement = line_segment_sketch.strokes[(index - 1).rem_euclid(L) as usize].displacement();
    let is_rotation_counterclockwise_result = previous_displacement.rotation_direction_to(&current_displacement);
    if is_rotation_counterclockwise_result == 0 {
        let mut previous_raw_cross = -999999.0f32;
        let previous_interval = line_segment_sketch.strokes[(index - 1).rem_euclid(L) as usize].points;
        for i1 in previous_interval.deleash().start()..previous_interval.deleash().end() {
            let displacement = line_segment_sketch.contour.deleash().displacement(previous_interval.deleash().start(), i1);
            previous_raw_cross = previous_raw_cross.max(current_displacement.cross(&displacement))
        }
        let mut current_raw_cross = -999999.0f32;
        let current_interval = line_segment_sketch.strokes[index.rem_euclid(L) as usize].points;
        for i2 in current_interval.deleash().start()..current_interval.deleash().end() {
            let displacement = line_segment_sketch.contour.deleash().displacement(previous_interval.deleash().start(), i2);
            current_raw_cross = current_raw_cross.max(current_displacement.cross(&displacement))
        }
        return current_raw_cross < previous_raw_cross;
    } else {
        return is_rotation_counterclockwise_result > 0;
    }
}