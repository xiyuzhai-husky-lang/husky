use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RawContour<'eval> {
    pub(crate) cc: &'eval crate::connected_component::ConnectedComponent,
    pub(crate) points: Vec<crate::geom2d::Point2d>,
}

impl<'eval> RawContour<'eval> {
    pub(crate) fn __call__(
        cc: &'eval crate::connected_component::ConnectedComponent,
        points: Vec<crate::geom2d::Point2d>,
    ) -> Self {
        Self { cc, points }
    }
    pub(crate) fn line_segment_sketch(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval crate::line_segment_sketch::LineSegmentSketch<'eval> {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::raw_contour::RawContour::line_segment_sketch"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<
                    crate::line_segment_sketch::LineSegmentSketch<'eval>,
                >(
                    crate::line_segment_sketch::LineSegmentSketch::new(&self, 1.4f32),
                    &__registration__::__LINE_SEGMENT_SKETCH_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
    }
    pub(crate) fn bounding_box(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval crate::geom2d::BoundingBox {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::raw_contour::RawContour::bounding_box"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__BOUNDING_BOX_VTABLE);
        }
        let start_point = &self.points[(0) as usize];
        let mut xmin = start_point.x;
        let mut xmax = start_point.x;
        let mut ymin = start_point.y;
        let mut ymax = start_point.y;
        for i in 0..self.points.ilen() {
            let point = &self.points[(i) as usize];
            xmin = xmin.min(point.x);
            xmax = xmax.max(point.x);
            ymin = ymin.min(point.y);
            ymax = ymax.max(point.y);
        }
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<crate::geom2d::BoundingBox>(
                    crate::geom2d::BoundingBox::__call__(
                        crate::geom2d::ClosedRange::__call__(xmin, xmax),
                        crate::geom2d::ClosedRange::__call__(ymin, ymax),
                    ),
                    &__registration__::__BOUNDING_BOX_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__BOUNDING_BOX_VTABLE)
    }
    pub(crate) fn relative_bounding_box(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval crate::geom2d::RelativeBoundingBox {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::raw_contour::RawContour::relative_bounding_box"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__RELATIVE_BOUNDING_BOX_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<crate::geom2d::RelativeBoundingBox>(
                    self.cc.raw_contours(__ctx)[(0) as usize]
                        .bounding_box(__ctx)
                        .relative_bounding_box(&self.bounding_box(__ctx)),
                    &__registration__::__RELATIVE_BOUNDING_BOX_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__RELATIVE_BOUNDING_BOX_VTABLE);
    }
    pub(crate) fn contour_len(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::raw_contour::RawContour::contour_len"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        let mut contour_len = 0f32;
        for i in (0 + 1)..self.points.ilen() {
            let a = &self.points[(i - 1) as usize];
            let b = &self.points[(i) as usize];
            contour_len += (a.x - b.x).abs() + (a.y - b.y).abs();
        }
        let a = &self.points[(self.points.ilen() - 1) as usize];
        let b = &self.points[(0) as usize];
        contour_len += (a.x - b.x).abs() + (a.y - b.y).abs();
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<f32>(
                    contour_len,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE)
    }
    pub(crate) fn displacement(&self, start: i32, end: i32) -> crate::geom2d::Vector2d {
        let N = self.points.ilen();
        let ct_start = &self.points[(start.rem_euclid(N)) as usize];
        let ct_end = &self.points[(end.rem_euclid(N)) as usize];
        return ct_start.to(&ct_end);
    }
}

impl<'eval> __StaticInfo for RawContour<'eval> {
    type __StaticSelf = RawContour<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::raw_contour::RawContour".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]

pub(crate) enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

impl From<i32> for Direction {
    fn from(__raw: i32) -> Self {
        match __raw {
            0 => Direction::UP,
            1 => Direction::LEFT,
            2 => Direction::DOWN,
            3 => Direction::RIGHT,
            _ => panic!(),
        }
    }
}
impl __StaticInfo for Direction {
    type __StaticSelf = Direction;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::raw_contour::Direction".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
pub(crate) fn get_pixel_pair(row: u32, j: i32) -> u32 {
    return (row >> (j - 1)) & 3u32;
}
pub(crate) fn get_pixel_to_the_left(row: u32, j: i32) -> u32 {
    return (row >> j) & 1u32;
}
pub(crate) fn get_pixel_to_the_right(row: u32, j: i32) -> u32 {
    return (row >> (j - 1)) & 1u32;
}
pub(crate) fn get_inward_direction(row_above: u32, row_below: u32, j: i32) -> Direction {
    let pixel_pair_above = get_pixel_pair(row_above, j);
    let pixel_pair_below = get_pixel_pair(row_below, j);
    match pixel_pair_above {
        0u32 => match pixel_pair_below {
            1u32 | 3u32 => {
                return Direction::LEFT;
            }
            2u32 => {
                return Direction::UP;
            }
            _ => panic!(),
        },
        1u32 => {
            return Direction::DOWN;
        }
        2u32 => match pixel_pair_below {
            0u32 => {
                return Direction::RIGHT;
            }
            1u32 | 3u32 => {
                return Direction::LEFT;
            }
            2u32 => {
                return Direction::UP;
            }
            _ => panic!(),
        },
        3u32 => match pixel_pair_below {
            0u32 | 1u32 => {
                return Direction::RIGHT;
            }
            2u32 => {
                return Direction::UP;
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}
pub(crate) fn get_angle_change(inward: Direction, outward: Direction) -> i32 {
    let raw_angle_change = (((outward as i32) - (inward as i32)) as u32).last_bits(2);
    match raw_angle_change {
        0u32 | 1u32 | 2u32 => {
            return raw_angle_change as i32;
        }
        3u32 => {
            return -1;
        }
        _ => panic!(),
    }
}
pub(crate) fn get_outward_direction(
    row_above: u32,
    row_below: u32,
    j: i32,
    inward_direction: Direction,
) -> Direction {
    let pixel_pair_above = get_pixel_pair(row_above, j);
    let pixel_pair_below = get_pixel_pair(row_below, j);
    match pixel_pair_above {
        0u32 => match pixel_pair_below {
            1u32 => {
                return Direction::DOWN;
            }
            2u32 | 3u32 => {
                return Direction::LEFT;
            }
            _ => panic!(),
        },
        1u32 => match pixel_pair_below {
            0u32 => {
                return Direction::RIGHT;
            }
            1u32 => {
                return Direction::DOWN;
            }
            2u32 => match inward_direction {
                Direction::DOWN => {
                    return Direction::LEFT;
                }
                Direction::UP => {
                    return Direction::RIGHT;
                }
                _ => panic!(),
            },
            3u32 => {
                return Direction::LEFT;
            }
            _ => panic!(),
        },
        2u32 => match pixel_pair_below {
            0u32 | 2u32 | 3u32 => {
                return Direction::UP;
            }
            1u32 => match inward_direction {
                Direction::LEFT => {
                    return Direction::UP;
                }
                Direction::RIGHT => {
                    return Direction::DOWN;
                }
                _ => panic!(),
            },
            _ => panic!(),
        },
        3u32 => match pixel_pair_below {
            0u32 | 2u32 => {
                return Direction::RIGHT;
            }
            1u32 => {
                return Direction::DOWN;
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StreakCache {
    pub(crate) prev1: i32,
    pub(crate) prev2: i32,
}

impl StreakCache {
    pub(crate) fn __call__(prev1: i32, prev2: i32) -> Self {
        Self { prev1, prev2 }
    }
}

impl __StaticInfo for StreakCache {
    type __StaticSelf = StreakCache;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::raw_contour::StreakCache".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
pub(crate) fn get_concave_middle_point(
    points: &Vec<crate::geom2d::Point2d>,
) -> crate::geom2d::Point2d {
    let N = points.ilen();
    let p0 = &points[(N - 2) as usize];
    let p2 = &points[(N - 1) as usize];
    return crate::geom2d::Point2d::__call__((p0.x + p2.x) / 2f32, (p0.y + p2.y) / 2f32);
}

pub(crate) fn find_raw_contours<'eval>(
    cc: &'eval crate::connected_component::ConnectedComponent,
) -> Vec<RawContour<'eval>> {
    let mut result = Vec::<RawContour>::__call__(vec![]);
    let mut boundary_unsearched = domains::ml::datasets::cv::mnist::BinaryGrid28::__call__();
    for i in 1..(29 + 1) {
        let r_ur = cc.mask[(i - 1) as usize];
        let r_dr = cc.mask[(i) as usize];
        let r_ul = r_ur << 1;
        let r_dl = r_dr << 1;
        boundary_unsearched[(i) as usize] =
            (r_ur | r_dr | r_ul | r_dl) & (!(r_ur & r_dr & r_ul & r_dl));
    }
    for k in 1..(29 + 1) {
        while boundary_unsearched[(k) as usize] != 0 {
            let mut contour = Vec::<crate::geom2d::Point2d>::__call__(vec![]);
            let mut i = k;
            let mut j = boundary_unsearched[(k) as usize].ctz();
            let mut row_above = cc.mask[(i - 1) as usize];
            let mut row_below = cc.mask[(i) as usize];
            let mut inward_direction = get_inward_direction(row_above, row_below, j);
            let i0 = i;
            let j0 = j;
            let dir0 = inward_direction;
            let mut prev_angle_change1 = 0;
            let mut prev_angle_change2 = 0;
            let mut total_angle_change = 0;
            let mut prev_streak1 = -1;
            let mut prev_streak2 = -1;
            let mut current_streak = -1;
            loop {
                let outward_direction =
                    get_outward_direction(row_above, row_below, j, inward_direction);
                let angle_change = get_angle_change(inward_direction, outward_direction);
                boundary_unsearched[(i) as usize] =
                    boundary_unsearched[(i) as usize] & (!(1u32 << j));
                if angle_change != 0 {
                    if prev_angle_change1 == -1
                        && prev_angle_change2 == -1
                        && current_streak == 1
                        && prev_streak1 != -1
                        && prev_streak2 == 1
                    {
                        *contour.lastx_mut() = get_concave_middle_point(&contour);
                        contour.push(crate::geom2d::Point2d::from_i_shift28(i, j));
                        prev_streak2 = -1;
                        prev_streak1 = -1;
                    } else if prev_angle_change1 == -1 && prev_streak1 > 0 && prev_streak1 == 1 {
                        *contour.lastx_mut() = crate::geom2d::Point2d::from_i_shift28(i, j);
                        prev_streak2 = prev_streak1;
                        prev_streak1 = current_streak;
                    } else if prev_angle_change1 == -1
                        && prev_streak1 > 0
                        && current_streak == 1
                        && prev_streak1 > 1
                    {
                        *contour.lastx_mut() = crate::geom2d::Point2d::from_i_shift28(i, j);
                        prev_streak2 = -1;
                        prev_streak1 = -1;
                    } else {
                        contour.push(crate::geom2d::Point2d::from_i_shift28(i, j));
                        prev_streak2 = prev_streak1;
                        prev_streak1 = current_streak;
                    }
                    current_streak = 0;
                    prev_angle_change2 = prev_angle_change1;
                    prev_angle_change1 = angle_change;
                }
                match outward_direction {
                    Direction::UP => {
                        i = i - 1;
                        row_below = row_above;
                        row_above = cc.mask[(i - 1) as usize];
                    }
                    Direction::DOWN => {
                        i = i + 1;
                        row_above = row_below;
                        row_below = cc.mask[(i) as usize];
                    }
                    Direction::LEFT => {
                        j = j + 1;
                    }
                    Direction::RIGHT => {
                        j = j - 1;
                    }
                    _ => panic!(),
                }
                inward_direction = outward_direction;
                if current_streak != -1 {
                    current_streak += 1;
                }
                if !(!(i == i0 && j == j0 && inward_direction == dir0)) {
                    break;
                }
            }
            if prev_angle_change1 == -1 && current_streak == 1 && prev_streak1 > 0 {
                contour.popx();
            }
            result.push(RawContour::__call__(&cc, contour));
        }
    }
    return result;
}
