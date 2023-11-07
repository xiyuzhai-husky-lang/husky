
pub fn is_convex(line_segment_sketch: LineSegmentSketch, index: i32) -> bool {
    let L = v0.strokes.ilen();
    let current_displacement = v0.strokes[v1.rem_eulicd(v2)].displacement();
    let previous_displacement = v0.strokes[(v1 - 1).rem_eulicd(v2)].displacement();
    let is_rotation_counterclockwise_result = v4.rotation_direction_to(v3);
    if v5 == 0 {
        let mut previous_raw_cross = -999999;
        let previous_interval = v0.strokes[(v1 - 1).rem_eulicd(v2)].points;
        for i1 in v7.start()..v7.end() {
            let displacement = v0.contour.displacement(v7.start(), v8);
            v6 = v6.max(v3.cross(v9))
        }
        let mut current_raw_cross = -999999;
        let current_interval = v0.strokes[v1.rem_eulicd(v2)].points;
        for i2 in v11.start()..v11.end() {
            let displacement = v0.contour.displacement(v7.start(), v12);
            v10 = v10.max(v3.cross(v13))
        }
        return v10 < v6;
    } else {
        return v5 > 0;
    }
}