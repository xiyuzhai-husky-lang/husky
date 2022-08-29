use crate::*;

pub(crate) mod concave_component;
pub(crate) mod convex_component;
pub(crate) mod convexity;
pub(crate) mod line_segment;
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LineSegmentStroke<'eval> {
    pub(crate) points: __std::slice::CyclicSlice<'eval, crate::geom2d::Point2d>,
    pub(crate) start: crate::geom2d::Point2d,
    pub(crate) end: crate::geom2d::Point2d,
}

impl<'eval> LineSegmentStroke<'eval> {
    pub(crate) fn __call__(
        points: __std::slice::CyclicSlice<'eval, crate::geom2d::Point2d>,
    ) -> Self {
        let start = points.firstx().clone();
        let end = points.lastx().clone();
        Self { points, start, end }
    }
    pub(crate) fn new(
        ct: &'eval crate::raw_contour::RawContour<'eval>,
        from: i32,
        to: i32,
    ) -> LineSegmentStroke<'eval> {
        assert!(from <= to);
        return LineSegmentStroke::__call__(ct.points.cyclic_slice(from, to + 1));
    }
    pub(crate) fn displacement(&self) -> crate::geom2d::Vector2d {
        return self.start.to(&self.end);
    }
}

impl<'eval> __StaticInfo for LineSegmentStroke<'eval> {
    type __StaticSelf = LineSegmentStroke<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::line_segment_sketch::LineSegmentStroke".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LineSegmentSketch<'eval> {
    pub(crate) contour: &'eval crate::raw_contour::RawContour<'eval>,
    pub(crate) strokes: Vec<LineSegmentStroke<'eval>>,
}

impl<'eval> LineSegmentSketch<'eval> {
    pub(crate) fn __call__(
        contour: &'eval crate::raw_contour::RawContour<'eval>,
        strokes: Vec<LineSegmentStroke<'eval>>,
    ) -> Self {
        Self { contour, strokes }
    }
    pub(crate) fn concave_components(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval Vec<concave_component::ConcaveComponent<'eval>> {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::line_segment_sketch::LineSegmentSketch::concave_components"
        );
        if let Some(__result) = __ctx.opt_cached_lazy_field(self as *const _ as *const (), __uid) {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const (),
                __uid,
                Ok(__Register::new_box::<
                    Vec<concave_component::ConcaveComponent<'eval>>,
                >(
                    concave_component::find_concave_components(&self),
                    &__registration__::__VEC_CONCAVE_COMPONENT_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
    }
    pub(crate) fn new(
        ct: &'eval crate::raw_contour::RawContour<'eval>,
        r: f32,
    ) -> LineSegmentSketch<'eval> {
        return LineSegmentSketch::__call__(&ct, find_line_segments(&ct, r));
    }
}

impl<'eval> __StaticInfo for LineSegmentSketch<'eval> {
    type __StaticSelf = LineSegmentSketch<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::line_segment_sketch::LineSegmentSketch".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
pub(crate) fn go_right(u: &crate::geom2d::Vector2d, r: f32) -> crate::geom2d::Vector2d {
    let L = (u.x * u.x + u.y * u.y).sqrt();
    assert!(L > r);
    let dr = r * L / (L * L - r * r).sqrt();
    let dx = dr * u.y / L;
    let dy = -dr * u.x / L;
    return crate::geom2d::Vector2d::__call__(u.x + dx, u.y + dy);
}
pub(crate) fn go_left(u: &crate::geom2d::Vector2d, r: f32) -> crate::geom2d::Vector2d {
    let L = (u.x * u.x + u.y * u.y).sqrt();
    assert!(L > r);
    let dr = r * L / (L * L - r * r).sqrt();
    let dx = -dr * u.y / L;
    let dy = dr * u.x / L;
    return crate::geom2d::Vector2d::__call__(u.x + dx, u.y + dy);
}

pub(crate) fn extend_end<'eval>(
    ct: &'eval crate::raw_contour::RawContour<'eval>,
    start: i32,
    r: f32,
) -> i32 {
    let mut end = start;
    let mut dp = ct.displacement(start, end + 1);
    let N = ct.points.ilen();
    let max_end = start + N;
    while end <= max_end && dp.norm() < r {
        end += 1;
        dp = ct.displacement(start, end + 1);
    }
    if dp.norm() < r {
        return end;
    }
    let mut right_bound = go_right(&dp, r);
    let mut left_bound = go_left(&dp, r);
    let mut r_max = 0f32;
    while end <= max_end
        && right_bound.rotation_direction_to(&dp) >= 0
        && dp.rotation_direction_to(&left_bound) >= 0
    {
        let dp_norm = dp.norm();
        if dp_norm < r_max - r {
            break;
        } else if dp_norm > r_max {
            r_max = dp_norm;
        }
        if dp_norm > r {
            let dp_right = go_right(&dp, r);
            let dp_left = go_left(&dp, r);
            if right_bound.rotation_direction_to(&dp_right) > 0 {
                right_bound = dp_right;
            }
            if dp_left.rotation_direction_to(&left_bound) > 0 {
                left_bound = dp_left;
            }
        }
        end += 1;
        dp = ct.displacement(start, end + 1);
    }
    assert!(end > start);
    return end;
}

pub(crate) fn extend_start<'eval>(
    ct: &'eval crate::raw_contour::RawContour<'eval>,
    start0: i32,
    end: i32,
    r: f32,
) -> i32 {
    let mut start = end;
    let mut dp0 = ct.displacement(end, start - 1);
    let min_start = end - ct.points.ilen();
    while start >= min_start && dp0.norm() < r {
        start -= 1;
        dp0 = ct.displacement(end, start - 1);
    }
    if dp0.norm() < r {
        return start.min(start0);
    }
    let mut right_bound = go_right(&dp0, r);
    let mut left_bound = go_left(&dp0, r);
    let mut r_max = 0f32;
    while start >= min_start {
        let dp = ct.displacement(end, start - 1);
        let dp_norm = dp.norm();
        if dp_norm < r_max - r {
            break;
        } else if dp_norm > r_max {
            r_max = dp_norm;
        }
        if dp_norm > r {
            let dp_right = go_right(&dp, r);
            let dp_left = go_left(&dp, r);
            if right_bound.rotation_direction_to(&dp_right) > 0 {
                right_bound = dp_right;
            }
            if dp_left.rotation_direction_to(&left_bound) > 0 {
                left_bound = dp_left;
            }
        }
        if right_bound.rotation_direction_to(&left_bound) >= 0 {
            if start <= start0
                && !(right_bound.rotation_direction_to(&dp) >= 0
                    && dp.rotation_direction_to(&left_bound) >= 0)
            {
                break;
            }
            start -= 1;
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

pub(crate) fn find_line_segments<'eval>(
    ct: &'eval crate::raw_contour::RawContour<'eval>,
    r: f32,
) -> Vec<LineSegmentStroke<'eval>> {
    let mut line_segments = Vec::<LineSegmentStroke>::__call__(vec![]);
    let mut start = 0;
    let mut end = 1;
    let mut max_end = ct.points.ilen();
    while end <= max_end {
        end = extend_end(&ct, start, r);
        let ls_extend_end = LineSegmentStroke::new(&ct, start, end);
        let mut extend_start_flag = true;
        if line_segments.ilen() > 0 {
            let dp_extend_end = ls_extend_end.displacement();
            let dp_previous = line_segments.lastx().displacement();
            if dp_extend_end.cross(&dp_previous).abs() < 0.01f32
                && dp_extend_end.dot(&dp_previous) > 0f32
            {
                let N = ct.points.ilen();
                *line_segments.lastx_mut() =
                    LineSegmentStroke::new(&ct, line_segments.lastx().points.start, end);
                extend_start_flag = false;
            }
        }
        if extend_start_flag {
            start = extend_start(&ct, start, end, r);
            let mut ls = LineSegmentStroke::new(&ct, start, end);
            if line_segments.ilen() > 0 {
                let ls_last = &line_segments.lastx();
                let dp_last = ls_last.displacement();
                let dp = ls.displacement();
                let dp1 = ls_last.start.to(&ls.end);
                if dp.cross(&dp_last).abs() < 0.001f32
                    && dp.dot(&dp_last) > 0f32
                    && dp.cross(&dp1).abs() < 0.001f32
                    && dp.dot(&dp1) > 0f32
                {
                    let ls_last = line_segments.popx();
                    ls = LineSegmentStroke::new(&ct, ls_last.points.start, ls.points.end);
                }
            } else {
                max_end = start + ct.points.ilen();
            }
            line_segments.push(ls);
        }
        start = end;
        end = start + 1;
    }
    let N = ct.points.ilen();
    let first_line_segment_points_end = line_segments.firstx().points.end;
    let last_line_segment = &line_segments.lastx();
    if last_line_segment.points.end >= first_line_segment_points_end + N {
        let last_line_segment = line_segments.popx();
        *line_segments.firstx_mut() = LineSegmentStroke::new(
            &ct,
            last_line_segment.points.start - N,
            line_segments.firstx().points.end - 1,
        );
    }
    return line_segments;
}
