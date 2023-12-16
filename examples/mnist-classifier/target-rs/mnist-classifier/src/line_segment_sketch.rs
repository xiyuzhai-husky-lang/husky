pub mod concave_component;
pub mod convex_component;
pub mod convexity;
pub mod line_segment;

pub use self::concave_component::*;
pub use self::convex_component::*;
pub use self::convexity::*;
pub use self::line_segment::*;

use crate::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineSegmentStroke {
    pub points: CyclicSliceLeashed<crate::geom2d::Point2d>,
    pub start: crate::geom2d::Point2d,
    pub end: crate::geom2d::Point2d,
}

impl LineSegmentStroke {
    pub fn __constructor(points: CyclicSliceLeashed<crate::geom2d::Point2d>) -> Self {
        let start = points.first().unwrap().clone();
        let end = points.last().unwrap().clone();
        Self{
            points,
            start,
            end,
        }
    }
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineSegmentSketch {
    pub contour: Leash<crate::raw_contour::RawContour>,
    pub strokes: Vec<crate::line_segment_sketch::LineSegmentStroke>,
}

impl LineSegmentSketch {
    pub fn __constructor(contour: Leash<crate::raw_contour::RawContour>, strokes: Vec<crate::line_segment_sketch::LineSegmentStroke>) -> Self {
        Self{
            contour,
            strokes,
        }
    }
}

#[rustfmt::skip]
pub fn go_right(u: &crate::geom2d::Vector2d, r: f32) -> crate::geom2d::Vector2d {
    let L = (u.x.into_inner() * u.x.into_inner() + u.y.into_inner() * u.y.into_inner()).sqrt();
    assert!(L > r);
    let dr = r * L / (L * L - r * r).sqrt();
    let dx = dr * u.y.into_inner() / L;
    let dy = -dr * u.x.into_inner() / L;
    crate::geom2d::Vector2d::__constructor(u.x.into_inner() + dx, u.y.into_inner() + dy)
}

#[rustfmt::skip]
pub fn go_left(u: &crate::geom2d::Vector2d, r: f32) -> crate::geom2d::Vector2d {
    let L = (u.x.into_inner() * u.x.into_inner() + u.y.into_inner() * u.y.into_inner()).sqrt();
    assert!(L > r);
    let dr = r * L / (L * L - r * r).sqrt();
    let dx = -dr * u.y.into_inner() / L;
    let dy = dr * u.x.into_inner() / L;
    crate::geom2d::Vector2d::__constructor(u.x.into_inner() + dx, u.y.into_inner() + dy)
}

#[rustfmt::skip]
pub fn extend_end(ct: Leash<crate::raw_contour::RawContour>, start: i32, r: f32) -> i32 {
    let mut end = start;
    let mut dp = ct.displacement(start, end + 1);
    let N = ct.points.ilen();
    let max_end = start + N;
    while end <= max_end && dp.norm() < r {
        end += 1;
        dp = ct.displacement(start, end + 1)
    }
    if dp.norm() < r {
        return end;
    }
    let mut right_bound = crate::line_segment_sketch::go_right(&dp, r);
    let mut left_bound = crate::line_segment_sketch::go_left(&dp, r);
    let mut r_max = 0.0f32;
    while end <= max_end && right_bound.rotation_direction_to(&dp) >= 0 && dp.rotation_direction_to(&left_bound) >= 0 {
        let dp_norm = dp.norm();
        if dp_norm < r_max - r {
            break;
        } else if dp_norm > r_max {
            r_max = dp_norm
        }
        if dp_norm > r {
            let dp_right = crate::line_segment_sketch::go_right(&dp, r);
            let dp_left = crate::line_segment_sketch::go_left(&dp, r);
            if right_bound.rotation_direction_to(&dp_right) > 0 {
                right_bound = dp_right
            }
            if dp_left.rotation_direction_to(&left_bound) > 0 {
                left_bound = dp_left
            }
        }
        end += 1;
        dp = ct.displacement(start, end + 1)
    }
    assert!(end > start);
    return end;
}

#[rustfmt::skip]
pub fn extend_start(ct: Leash<crate::raw_contour::RawContour>, start0: i32, end: i32, r: f32) -> i32 {
    let mut start = end;
    let mut dp0 = ct.displacement(end, start - 1);
    let min_start = end - ct.points.ilen();
    while start >= min_start && dp0.norm() < r {
        start -= 1;
        dp0 = ct.displacement(end, start - 1)
    }
    if dp0.norm() < r {
        return start.min(start0);
    }
    let mut right_bound = crate::line_segment_sketch::go_right(&dp0, r);
    let mut left_bound = crate::line_segment_sketch::go_left(&dp0, r);
    let mut r_max = 0.0f32;
    while start >= min_start {
        let dp = ct.displacement(end, start - 1);
        let dp_norm = dp.norm();
        if dp_norm < r_max - r {
            break;
        } else if dp_norm > r_max {
            r_max = dp_norm
        }
        if dp_norm > r {
            let dp_right = crate::line_segment_sketch::go_right(&dp, r);
            let dp_left = crate::line_segment_sketch::go_left(&dp, r);
            if right_bound.rotation_direction_to(&dp_right) > 0 {
                right_bound = dp_right
            }
            if dp_left.rotation_direction_to(&left_bound) > 0 {
                left_bound = dp_left
            }
        }
        if right_bound.rotation_direction_to(&left_bound) >= 0 {
            if start <= start0 && !(right_bound.rotation_direction_to(&dp) >= 0 && dp.rotation_direction_to(&left_bound) >= 0) {
                break;
            }
            start -= 1
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

#[rustfmt::skip]
pub fn find_line_segments(ct: Leash<crate::raw_contour::RawContour>, r: f32) -> Vec<crate::line_segment_sketch::LineSegmentStroke> {
    let mut line_segments: Vec<crate::line_segment_sketch::LineSegmentStroke> = vec![];
    let mut start = 0;
    let mut end = 1;
    let mut max_end = ct.points.ilen();
    while end <= max_end {
        end = crate::line_segment_sketch::extend_end(ct, start, r);
        let ls_extend_end = crate::line_segment_sketch::LineSegmentStroke::new(ct, start, end);
        let mut extend_start_flag = true;
        if line_segments.ilen() > 0 {
            let dp_extend_end = ls_extend_end.displacement();
            let dp_previous = line_segments.last().unwrap().displacement();
            if dp_extend_end.cross(&dp_previous).abs() < 0.01f32 && dp_extend_end.dot(&dp_previous) > 0.0f32 {
                let N = ct.points.ilen();
                *line_segments.last_mut().unwrap() = crate::line_segment_sketch::LineSegmentStroke::new(ct, line_segments.last().unwrap().points.start(), end);
                extend_start_flag = false
            }
        }
        if extend_start_flag {
            start = crate::line_segment_sketch::extend_start(ct, start, end, r);
            let mut ls = crate::line_segment_sketch::LineSegmentStroke::new(ct, start, end);
            if line_segments.ilen() > 0 {
                let ls_last = line_segments.last().unwrap();
                let dp_last = ls_last.displacement();
                let dp = ls.displacement();
                let dp1 = ls_last.start.to(&ls.end);
                if dp.cross(&dp_last).abs() < 0.001f32 && dp.dot(&dp_last) > 0.0f32 && dp.cross(&dp1).abs() < 0.001f32 && dp.dot(&dp1) > 0.0f32 {
                    let ls_last = line_segments.pop().unwrap();
                    ls = crate::line_segment_sketch::LineSegmentStroke::new(ct, ls_last.points.start(), ls.points.end())
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
        *line_segments.first_mut().unwrap() = crate::line_segment_sketch::LineSegmentStroke::new(ct, last_line_segment.points.start() - N, line_segments.first().unwrap().points.end() - 1)
    }
    line_segments
}

#[rustfmt::skip]
impl crate::line_segment_sketch::LineSegmentStroke {
    pub fn new(ct: Leash<crate::raw_contour::RawContour>, from: i32, to: i32) -> crate::line_segment_sketch::LineSegmentStroke {
        assert!(from <= to);
        crate::line_segment_sketch::LineSegmentStroke::__constructor(ct.points.cyclic_slice_leashed(from, to + 1))
    }

    pub fn displacement(&self) -> crate::geom2d::Vector2d {
        self.start.to(&self.end)
    }
}

#[rustfmt::skip]
impl crate::line_segment_sketch::LineSegmentSketch {
    #[ad_hoc_task_dependency::memoized_field_return_ref(13)]
pub fn concave_components(&'static self) -> Vec<crate::line_segment_sketch::concave_component::ConcaveComponent> {
        crate::line_segment_sketch::concave_component::find_concave_components(&self)
    }

    #[ad_hoc_task_dependency::memoized_field_return_ref(14)]
pub fn bounding_box(&'static self) -> crate::geom2d::BoundingBox {
        let start_point = &self.strokes[0 as usize].start;
        let mut xmin = start_point.x.into_inner();
        let mut xmax = start_point.x.into_inner();
        let mut ymin = start_point.y.into_inner();
        let mut ymax = start_point.y.into_inner();
        for i in 0..self.strokes.ilen() {
            let point = &self.strokes[i as usize].end;
            xmin = xmin.min(point.x.into_inner());
            xmax = xmax.max(point.x.into_inner());
            ymin = ymin.min(point.y.into_inner());
            ymax = ymax.max(point.y.into_inner())
        }
        return crate::geom2d::BoundingBox::__constructor(crate::geom2d::ClosedRange::__constructor(xmin, xmax), crate::geom2d::ClosedRange::__constructor(ymin, ymax));
    }

    pub fn new(ct: Leash<crate::raw_contour::RawContour>, r: f32) -> crate::line_segment_sketch::LineSegmentSketch {
        crate::line_segment_sketch::LineSegmentSketch::__constructor(ct, crate::line_segment_sketch::find_line_segments(ct, r))
    }
}