
pub mod concave_component;

pub mod convex_component;

pub mod convexity;

pub mod line_segment;

struct LineSegmentStroke{points: Leash<CyclicSlice<Point2d>>, start: Point2d, end: Point2d}

struct LineSegmentSketch{contour: Leash<RawContour>, strokes: Vec<LineSegmentStroke>}

pub fn go_right(u: Vector2d, r: f32) -> Vector2d {
    let L = (v0.x * v0.x + v0.y * v0.y).sqrt();
    assert!(v2 > v1);
    let dr = v1 * v2 / (v2 * v2 - v1 * v1).sqrt();
    let dx = v3 * v0.y / v2;
    let dy = -v3 * v0.x / v2;
    Vector2d(v0.x + v4, v0.y + v5)
}

pub fn go_left(u: Vector2d, r: f32) -> Vector2d {
    let L = (v0.x * v0.x + v0.y * v0.y).sqrt();
    assert!(v2 > v1);
    let dr = v1 * v2 / (v2 * v2 - v1 * v1).sqrt();
    let dx = -v3 * v0.y / v2;
    let dy = v3 * v0.x / v2;
    Vector2d(v0.x + v4, v0.y + v5)
}

pub fn extend_end(ct: Leash<RawContour>, start: i32, r: f32) -> i32 {
    let mut end = v1;
    let mut dp = v0.displacement(v1, v3 + 1);
    let N = v0.points.ilen();
    let max_end = v1 + v5;
    while v3 <= v6 && v4.norm() < v2 {
        v3+= 1;
        v4 = v0.displacement(v1, v3 + 1)
    }
    if v4.norm() < v2 {
        return v3;
    }
    let mut right_bound = go_right(v4, v2);
    let mut left_bound = go_left(v4, v2);
    let mut r_max = 0;
    while v3 <= v6 && v7.rotation_direction_to(v4) >= 0 && v4.rotation_direction_to(v8) >= 0 {
        let dp_norm = v4.norm();
        if v10 < v9 - v2 {
            break;
        } else if v10 > v9 {
            v9 = v10
        }
        if v10 > v2 {
            let dp_right = go_right(v4, v2);
            let dp_left = go_left(v4, v2);
            if v7.rotation_direction_to(v11) > 0 {
                v7 = v11
            }
            if v12.rotation_direction_to(v8) > 0 {
                v8 = v12
            }
        }
        v3+= 1;
        v4 = v0.displacement(v1, v3 + 1)
    }
    assert!(v3 > v1);
    return v3;
}

pub fn extend_start(ct: Leash<RawContour>, start0: i32, end: i32, r: f32) -> i32 {
    let mut start = v2;
    let mut dp0 = v0.displacement(v2, v4 - 1);
    let min_start = v2 - v0.points.ilen();
    while v4 >= v6 && v5.norm() < v3 {
        v4-= 1;
        v5 = v0.displacement(v2, v4 - 1)
    }
    if v5.norm() < v3 {
        return v4.min(v1);
    }
    let mut right_bound = go_right(v5, v3);
    let mut left_bound = go_left(v5, v3);
    let mut r_max = 0;
    while v4 >= v6 {
        let dp = v0.displacement(v2, v4 - 1);
        let dp_norm = v10.norm();
        if v11 < v9 - v3 {
            break;
        } else if v11 > v9 {
            v9 = v11
        }
        if v11 > v3 {
            let dp_right = go_right(v10, v3);
            let dp_left = go_left(v10, v3);
            if v7.rotation_direction_to(v12) > 0 {
                v7 = v12
            }
            if v13.rotation_direction_to(v8) > 0 {
                v8 = v13
            }
        }
        if v7.rotation_direction_to(v8) >= 0 {
            if v4 <= v1 && !(v7.rotation_direction_to(v10) >= 0 && v10.rotation_direction_to(v8) >= 0) {
                break;
            }
            v4-= 1
        } else {
            break;
        }
    }
    if v4 <= v1 {
        return v4;
    } else {
        return v1;
    }
}

pub fn find_line_segments(ct: Leash<RawContour>, r: f32) -> Vec<LineSegmentStroke> {
    let mut line_segments: Vec<LineSegmentStroke> = vec![];
    let mut start = 0;
    let mut end = 1;
    let mut max_end = v0.points.ilen();
    while v4 <= v5 {
        v4 = extend_end(v0, v3, v1);
        let ls_extend_end = new(v0, v3, v4);
        let mut extend_start_flag = true;
        if v2.ilen() > 0 {
            let dp_extend_end = v6.displacement();
            let dp_previous = v2.last().unwrap().displacement();
            if v8.cross(v9).abs() < 0.01 && v8.dot(v9) > 0 {
                let N = v0.points.ilen();
                v2.last().unwrap() = new(v0, v2.last().unwrap().points.start(), v4);
                v7 = false
            }
        }
        if v7 {
            v3 = extend_start(v0, v3, v4, v1);
            let mut ls = new(v0, v3, v4);
            if v2.ilen() > 0 {
                let ls_last = v2.last().unwrap();
                let dp_last = v12.displacement();
                let dp = v11.displacement();
                let dp1 = v12.start.to(v11.end);
                if v14.cross(v13).abs() < 0.001 && v14.dot(v13) > 0 && v14.cross(v15).abs() < 0.001 && v14.dot(v15) > 0 {
                    let ls_last = v2.pop().unwrap();
                    v11 = new(v0, v16.points.start(), v11.points.end())
                }
            } else {
                v5 = v3 + v0.points.ilen()
            }
            v2.push(v11)
        }
        v3 = v4;
        v4 = v3 + 1
    }
    let N = v0.points.ilen();
    let first_line_segment_points_end = v2.first().unwrap().points.end();
    let last_line_segment = v2.last().unwrap();
    if v19.points.end() >= v18 + v17 {
        let last_line_segment = v2.pop().unwrap();
        v2.first().unwrap() = new(v0, v20.points.start() - v17, v2.first().unwrap().points.end() - 1)
    }
    v2
}