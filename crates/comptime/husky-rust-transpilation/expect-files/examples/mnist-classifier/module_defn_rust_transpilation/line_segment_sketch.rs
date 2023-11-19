pub mod concave_component;
pub mod convex_component;
pub mod convexity;
pub mod line_segment;

use self::concave_component::*;
use self::convex_component::*;
use self::convexity::*;
use self::line_segment::*;

use crate::*;

pub struct LineSegmentStroke{
    points: Leash<CyclicSlice<Point2d>>,
    start: Point2d,
    end: Point2d,
}

pub struct LineSegmentSketch{
    contour: Leash<RawContour>,
    strokes: Vec<LineSegmentStroke>,
}

pub fn go_right(u: Vector2d, r: f32) -> Vector2d {
    let L = (u.x * u.x + u.y * u.y).sqrt();
    assert!(L > r);
    let dr = r * L / (L * L - r * r).sqrt();
    let dx = dr * u.y / L;
    let dy = -dr * u.x / L;
    Vector2d(u.x + dx, u.y + dy)
}

pub fn go_left(u: Vector2d, r: f32) -> Vector2d {
    let L = (u.x * u.x + u.y * u.y).sqrt();
    assert!(L > r);
    let dr = r * L / (L * L - r * r).sqrt();
    let dx = -dr * u.y / L;
    let dy = dr * u.x / L;
    Vector2d(u.x + dx, u.y + dy)
}

pub fn extend_end(ct: Leash<RawContour>, start: i32, r: f32) -> i32 {
    let mut end = start;
    let mut dp = ct.displacement(start, end + 1);
    let N = ct.points.ilen();
    let max_end = start + N;
    while end <= max_end && dp.norm() < r {
        end+= 1;
        dp = ct.displacement(start, end + 1)
    }
    if dp.norm() < r {
        return end;
    }
    let mut right_bound = go_right(dp, r);
    let mut left_bound = go_left(dp, r);
    let mut r_max = 0;
    while end <= max_end && right_bound.rotation_direction_to(dp) >= 0 && dp.rotation_direction_to(left_bound) >= 0 {
        let dp_norm = dp.norm();
        if dp_norm < r_max - r {
            break;
        } else if dp_norm > r_max {
            r_max = dp_norm
        }
        if dp_norm > r {
            let dp_right = go_right(dp, r);
            let dp_left = go_left(dp, r);
            if right_bound.rotation_direction_to(dp_right) > 0 {
                right_bound = dp_right
            }
            if dp_left.rotation_direction_to(left_bound) > 0 {
                left_bound = dp_left
            }
        }
        end+= 1;
        dp = ct.displacement(start, end + 1)
    }
    assert!(end > start);
    return end;
}

pub fn extend_start(ct: Leash<RawContour>, start0: i32, end: i32, r: f32) -> i32 {
    let mut start = end;
    let mut dp0 = ct.displacement(end, start - 1);
    let min_start = end - ct.points.ilen();
    while start >= min_start && dp0.norm() < r {
        start-= 1;
        dp0 = ct.displacement(end, start - 1)
    }
    if dp0.norm() < r {
        return start.min(start0);
    }
    let mut right_bound = go_right(dp0, r);
    let mut left_bound = go_left(dp0, r);
    let mut r_max = 0;
    while start >= min_start {
        let dp = ct.displacement(end, start - 1);
        let dp_norm = dp.norm();
        if dp_norm < r_max - r {
            break;
        } else if dp_norm > r_max {
            r_max = dp_norm
        }
        if dp_norm > r {
            let dp_right = go_right(dp, r);
            let dp_left = go_left(dp, r);
            if right_bound.rotation_direction_to(dp_right) > 0 {
                right_bound = dp_right
            }
            if dp_left.rotation_direction_to(left_bound) > 0 {
                left_bound = dp_left
            }
        }
        if right_bound.rotation_direction_to(left_bound) >= 0 {
            if start <= start0 && !(right_bound.rotation_direction_to(dp) >= 0 && dp.rotation_direction_to(left_bound) >= 0) {
                break;
            }
            start-= 1
        } else {
            break;
        }
    }
    if start <= start0 {
        return start;
    } else {
        return start0;
    }
}

pub fn find_line_segments(ct: Leash<RawContour>, r: f32) -> Vec<LineSegmentStroke> {
    let mut line_segments: Vec<LineSegmentStroke> = vec![];
    let mut start = 0;
    let mut end = 1;
    let mut max_end = ct.points.ilen();
    while end <= max_end {
        end = extend_end(ct, start, r);
        let ls_extend_end = LineSegmentStroke::new(ct, start, end);
        let mut extend_start_flag = true;
        if line_segments.ilen() > 0 {
            let dp_extend_end = ls_extend_end.displacement();
            let dp_previous = line_segments.last().unwrap().displacement();
            if dp_extend_end.cross(dp_previous).abs() < 0.01 && dp_extend_end.dot(dp_previous) > 0 {
                let N = ct.points.ilen();
                line_segments.last().unwrap() = LineSegmentStroke::new(ct, line_segments.last().unwrap().points.start(), end);
                extend_start_flag = false
            }
        }
        if extend_start_flag {
            start = extend_start(ct, start, end, r);
            let mut ls = LineSegmentStroke::new(ct, start, end);
            if line_segments.ilen() > 0 {
                let ls_last = line_segments.last().unwrap();
                let dp_last = ls_last.displacement();
                let dp = ls.displacement();
                let dp1 = ls_last.start.to(ls.end);
                if dp.cross(dp_last).abs() < 0.001 && dp.dot(dp_last) > 0 && dp.cross(dp1).abs() < 0.001 && dp.dot(dp1) > 0 {
                    let ls_last = line_segments.pop().unwrap();
                    ls = LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())
                }
            } else {
                max_end = start + ct.points.ilen()
            }
            line_segments.push(ls)
        }
        start = end;
        end = start + 1
    }
    let N = ct.points.ilen();
    let first_line_segment_points_end = line_segments.first().unwrap().points.end();
    let last_line_segment = line_segments.last().unwrap();
    if last_line_segment.points.end() >= first_line_segment_points_end + N {
        let last_line_segment = line_segments.pop().unwrap();
        line_segments.first().unwrap() = LineSegmentStroke::new(ct, last_line_segment.points.start() - N, line_segments.first().unwrap().points.end() - 1)
    }
    line_segments
}

impl Visualize for LineSegmentStroke {
    fn visualize(self) {
        
    }
}

impl LineSegmentStroke {
    fn new(ct: Leash<RawContour>, from: i32, to: i32) {
        assert!(from <= to);
        LineSegmentStroke(ct.points.cyclic_slice_leashed(from, to + 1))
    }

    fn displacement(self) {
        self.start.to(self.end)
    }
}

impl Visualize for LineSegmentSketch {
    fn visualize(self) {
        self.strokes.visualize()
    }
}

impl LineSegmentSketch {
    fn concave_components(self) {
        find_concave_components(self)
    }

    fn bounding_box(self) {
        let start_point = self.strokes[0].start;
        let mut xmin = start_point.x;
        let mut xmax = start_point.x;
        let mut ymin = start_point.y;
        let mut ymax = start_point.y;
        for i in 0..self.strokes.ilen() {
            let point = self.strokes[i].end;
            xmin = xmin.min(point.x);
            xmax = xmax.max(point.x);
            ymin = ymin.min(point.y);
            ymax = ymax.max(point.y)
        }
        return BoundingBox(ClosedRange(xmin, xmax), ClosedRange(ymin, ymax));
    }

    fn new(ct: Leash<RawContour>, r: f32) {
        LineSegmentSketch(ct, find_line_segments(ct, r))
    }
}