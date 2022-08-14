use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::connected_component::major_connected_component"
        },
        feature_linkage!(connected_component::major_connected_component, __registration__::__CONNECTED_COMPONENT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::ilen" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<line_segment_sketch::concave_component::ConcaveComponent>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::connected_component::ConnectedComponent"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let mask: domains::ml::datasets::cv::mnist::BinaryImage28 = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__BINARY_IMAGE_28_VTABLE);
                    __Register::new_box(connected_component::ConnectedComponent::__call__(mask), &__registration__::__CONNECTED_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(connected_component::ConnectedComponent::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "mask",
        },
        eager_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, __registration__::__BINARY_IMAGE_28_VTABLE, mask, invalid)
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::connected_component::ConnectedComponent::raw_contours",
        },
        lazy_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, __registration__::__VEC_RAW_CONTOUR_VTABLE, raw_contours)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::raw_contour::RawContour>::ilen" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<raw_contour::RawContour<'eval>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_RAW_CONTOUR_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<raw_contour::RawContour>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::raw_contour::RawContour"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let cc: &'eval connected_component::ConnectedComponent = __arguments[0].downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
                let points: Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__VEC_POINT_2_D_VTABLE);
                    __Register::new_box(raw_contour::RawContour::__call__(cc, points), &__registration__::__RAW_CONTOUR_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::RawContour::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "cc",
        },
        eager_field_linkage!(raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE, __registration__::__CONNECTED_COMPONENT_VTABLE, cc, box)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "points",
        },
        eager_field_linkage!(raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE, __registration__::__VEC_POINT_2_D_VTABLE, points, invalid)
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::raw_contour::RawContour::line_segment_sketch",
        },
        lazy_field_linkage!(raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, line_segment_sketch)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::raw_contour::RawContour"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                                let __variadics =
                                    __arguments[0..]
                                        .iter_mut()
                                        .map(|v|v.downcast_move(&__registration__::__RAW_CONTOUR_VTABLE))
                                        .collect();
                    __Register::new_box(Vec::<raw_contour::RawContour>::__call__(__variadics), &__registration__::__VEC_RAW_CONTOUR_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<raw_contour::RawContour>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::raw_contour::RawContour", "i32"],
        },
        index_linkage!(Vec<raw_contour::RawContour<'eval>>,
    __registration__::__VEC_RAW_CONTOUR_VTABLE,
    __registration__::__RAW_CONTOUR_VTABLE,
    invalid,
    mutable
)
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::connected_component::connected_components"
        },
        feature_linkage!(connected_component::connected_components, __registration__::__VEC_CONNECTED_COMPONENT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::connected_component::ConnectedComponent>::ilen" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<connected_component::ConnectedComponent> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_CONNECTED_COMPONENT_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<connected_component::ConnectedComponent>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::connected_component::ConnectedComponent"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                                let __variadics =
                                    __arguments[0..]
                                        .iter_mut()
                                        .map(|v|v.downcast_move(&__registration__::__CONNECTED_COMPONENT_VTABLE))
                                        .collect();
                    __Register::new_box(Vec::<connected_component::ConnectedComponent>::__call__(__variadics), &__registration__::__VEC_CONNECTED_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<connected_component::ConnectedComponent>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::connected_component::ConnectedComponent", "i32"],
        },
        index_linkage!(Vec<connected_component::ConnectedComponent>,
    __registration__::__VEC_CONNECTED_COMPONENT_VTABLE,
    __registration__::__CONNECTED_COMPONENT_VTABLE,
    invalid,
    mutable
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::connected_component::find_connected_components",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let img: &domains::ml::datasets::cv::mnist::BinaryImage28 = __arguments[0].downcast_temp_ref(&__registration__::__BINARY_IMAGE_28_VTABLE);
                    __Register::new_box(connected_component::find_connected_components(img), &__registration__::__VEC_CONNECTED_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(connected_component::find_connected_components as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::connected_component::horizontal_extend",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let a: u32 = __arguments[0].downcast_b32();
                let x: u32 = __arguments[1].downcast_b32();
                    connected_component::horizontal_extend(a, x).to_register()
                }
                __wrapper
            },
            opt_fp: Some(connected_component::horizontal_extend as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::connected_component::ConnectedComponent>::push" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<connected_component::ConnectedComponent> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_CONNECTED_COMPONENT_VTABLE);
                let element: connected_component::ConnectedComponent = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CONNECTED_COMPONENT_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<connected_component::ConnectedComponent>::push as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::ilen" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<geom2d::Point2d> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_POINT_2_D_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<geom2d::Point2d>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::Point2d"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let x: f32 = __arguments[0].downcast_f32();
                let y: f32 = __arguments[1].downcast_f32();
                    __Register::new_box(geom2d::Point2d::__call__(x, y), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(geom2d::Point2d::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::geom2d::Point2d",
            field_ident: "x",
        },
        eager_field_linkage!(geom2d::Point2d, __registration__::__POINT_2_D_VTABLE, __registration__::__F32_VTABLE, x, direct)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::geom2d::Point2d",
            field_ident: "y",
        },
        eager_field_linkage!(geom2d::Point2d, __registration__::__POINT_2_D_VTABLE, __registration__::__F32_VTABLE, y, direct)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::geom2d::Point2d"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                                let __variadics =
                                    __arguments[0..]
                                        .iter_mut()
                                        .map(|v|v.downcast_move(&__registration__::__POINT_2_D_VTABLE))
                                        .collect();
                    __Register::new_box(Vec::<geom2d::Point2d>::__call__(__variadics), &__registration__::__VEC_POINT_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<geom2d::Point2d>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::geom2d::Point2d", "i32"],
        },
        index_linkage!(Vec<geom2d::Point2d>,
    __registration__::__VEC_POINT_2_D_VTABLE,
    __registration__::__POINT_2_D_VTABLE,
    invalid,
    mutable
)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let contour: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                let line_segments: Vec<line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__VEC_LINE_SEGMENT_VTABLE);
                    __Register::new_box(line_segment_sketch::LineSegmentSketch::__call__(contour, line_segments), &__registration__::__LINE_SEGMENT_SKETCH_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::LineSegmentSketch::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "contour",
        },
        eager_field_linkage!(line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, __registration__::__RAW_CONTOUR_VTABLE, contour, box)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "line_segments",
        },
        eager_field_linkage!(line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, __registration__::__VEC_LINE_SEGMENT_VTABLE, line_segments, invalid)
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::line_segment_sketch::LineSegmentSketch::concave_components",
        },
        lazy_field_linkage!(line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, __registration__::__VEC_CONCAVE_COMPONENT_VTABLE, concave_components)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_pixel_pair",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let row: u32 = __arguments[0].downcast_b32();
                let j: i32 = __arguments[1].downcast_i32();
                    raw_contour::get_pixel_pair(row, j).to_register()
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::get_pixel_pair as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_pixel_to_the_left",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let row: u32 = __arguments[0].downcast_b32();
                let j: i32 = __arguments[1].downcast_i32();
                    raw_contour::get_pixel_to_the_left(row, j).to_register()
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::get_pixel_to_the_left as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_pixel_to_the_right",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let row: u32 = __arguments[0].downcast_b32();
                let j: i32 = __arguments[1].downcast_i32();
                    raw_contour::get_pixel_to_the_right(row, j).to_register()
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::get_pixel_to_the_right as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_inward_direction",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let row_above: u32 = __arguments[0].downcast_b32();
                let row_below: u32 = __arguments[1].downcast_b32();
                let j: i32 = __arguments[2].downcast_i32();
                    __Register::new_box(raw_contour::get_inward_direction(row_above, row_below, j), &__registration__::__DIRECTION_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::get_inward_direction as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_angle_change",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let inward: raw_contour::Direction = __arguments[0].downcast_temp_ref::<__VirtualEnum>(&__registration__::__VIRTUAL_ENUM_VTABLE).kind_idx.into();
                let outward: raw_contour::Direction = __arguments[1].downcast_temp_ref::<__VirtualEnum>(&__registration__::__VIRTUAL_ENUM_VTABLE).kind_idx.into();
                    raw_contour::get_angle_change(inward, outward).to_register()
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::get_angle_change as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_outward_direction",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let row_above: u32 = __arguments[0].downcast_b32();
                let row_below: u32 = __arguments[1].downcast_b32();
                let j: i32 = __arguments[2].downcast_i32();
                let inward_direction: raw_contour::Direction = __arguments[3].downcast_temp_ref::<__VirtualEnum>(&__registration__::__VIRTUAL_ENUM_VTABLE).kind_idx.into();
                    __Register::new_box(raw_contour::get_outward_direction(row_above, row_below, j, inward_direction), &__registration__::__DIRECTION_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::get_outward_direction as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::raw_contour::StreakCache"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let prev1: i32 = __arguments[0].downcast_i32();
                let prev2: i32 = __arguments[1].downcast_i32();
                    __Register::new_box(raw_contour::StreakCache::__call__(prev1, prev2), &__registration__::__STREAK_CACHE_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::StreakCache::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::raw_contour::StreakCache",
            field_ident: "prev1",
        },
        eager_field_linkage!(raw_contour::StreakCache, __registration__::__STREAK_CACHE_VTABLE, __registration__::__I32_VTABLE, prev1, direct)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::raw_contour::StreakCache",
            field_ident: "prev2",
        },
        eager_field_linkage!(raw_contour::StreakCache, __registration__::__STREAK_CACHE_VTABLE, __registration__::__I32_VTABLE, prev2, direct)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_concave_middle_point",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let points: &Vec<geom2d::Point2d> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_POINT_2_D_VTABLE);
                    __Register::new_box(raw_contour::get_concave_middle_point(points), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::get_concave_middle_point as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::find_raw_contours",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let cc: &'eval connected_component::ConnectedComponent = __arguments[0].downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
                    __Register::new_box(raw_contour::find_raw_contours(cc), &__registration__::__VEC_RAW_CONTOUR_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::find_raw_contours as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::lastx" },
        method_elem_linkage!(Vec<geom2d::Point2d>, __registration__::__VEC_POINT_2_D_VTABLE, __registration__::__POINT_2_D_VTABLE, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::geom2d::Point2d::from_i_shift28",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let i: i32 = __arguments[0].downcast_i32();
                let shift: i32 = __arguments[1].downcast_i32();
                    __Register::new_box(geom2d::Point2d::from_i_shift28(i, shift), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(geom2d::Point2d::from_i_shift28 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::push" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_POINT_2_D_VTABLE);
                let element: geom2d::Point2d = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__POINT_2_D_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<geom2d::Point2d>::push as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::popx" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_POINT_2_D_VTABLE);
                    __Register::new_box(__this.popx(), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<geom2d::Point2d>::popx as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::raw_contour::RawContour>::push" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<raw_contour::RawContour<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_RAW_CONTOUR_VTABLE);
                let element: raw_contour::RawContour<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__RAW_CONTOUR_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<raw_contour::RawContour>::push as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::Vector2d"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let x: f32 = __arguments[0].downcast_f32();
                let y: f32 = __arguments[1].downcast_f32();
                    __Register::new_box(geom2d::Vector2d::__call__(x, y), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(geom2d::Vector2d::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::geom2d::Vector2d",
            field_ident: "x",
        },
        eager_field_linkage!(geom2d::Vector2d, __registration__::__VECTOR_2_D_VTABLE, __registration__::__F32_VTABLE, x, direct)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::geom2d::Vector2d",
            field_ident: "y",
        },
        eager_field_linkage!(geom2d::Vector2d, __registration__::__VECTOR_2_D_VTABLE, __registration__::__F32_VTABLE, y, direct)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let line_segment_sketch: &'eval line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
                let line_segments: __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CYCLIC_SLICE_LINE_SEGMENT_VTABLE);
                    __Register::new_box(line_segment_sketch::concave_component::ConcaveComponent::__call__(line_segment_sketch, line_segments), &__registration__::__CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::concave_component::ConcaveComponent::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "line_segment_sketch",
        },
        eager_field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, line_segment_sketch, box)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "line_segments",
        },
        eager_field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_VTABLE, line_segments, invalid)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::LineSegment"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let points: __std::slice::CyclicSlice<'eval, geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE);
                    __Register::new_box(line_segment_sketch::LineSegment::__call__(points), &__registration__::__LINE_SEGMENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::LineSegment::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegment",
            field_ident: "points",
        },
        eager_field_linkage!(line_segment_sketch::LineSegment<'eval>, __registration__::__LINE_SEGMENT_VTABLE, __registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE, points, invalid)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegment",
            field_ident: "start",
        },
        eager_field_linkage!(line_segment_sketch::LineSegment<'eval>, __registration__::__LINE_SEGMENT_VTABLE, __registration__::__POINT_2_D_VTABLE, start, invalid)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegment",
            field_ident: "end",
        },
        eager_field_linkage!(line_segment_sketch::LineSegment<'eval>, __registration__::__LINE_SEGMENT_VTABLE, __registration__::__POINT_2_D_VTABLE, end, invalid)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>",
            field_ident: "start",
        },
        eager_mut_field_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>>, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_VTABLE, __registration__::__I32_VTABLE, start, direct)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>",
            field_ident: "end",
        },
        eager_mut_field_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>>, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_VTABLE, __registration__::__I32_VTABLE, end, direct)
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>", "i32"],
        },
        index_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>>,
    __registration__::__CYCLIC_SLICE_LINE_SEGMENT_VTABLE,
    __registration__::__LINE_SEGMENT_VTABLE,
    invalid,
    immutable
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::concave_component::find_concave_components",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let line_segment_sketch: &'eval line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
                    __Register::new_box(line_segment_sketch::concave_component::find_concave_components(line_segment_sketch), &__registration__::__VEC_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::concave_component::find_concave_components as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                                let __variadics =
                                    __arguments[0..]
                                        .iter_mut()
                                        .map(|v|v.downcast_move(&__registration__::__CONCAVE_COMPONENT_VTABLE))
                                        .collect();
                    __Register::new_box(Vec::<line_segment_sketch::concave_component::ConcaveComponent>::__call__(__variadics), &__registration__::__VEC_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<line_segment_sketch::concave_component::ConcaveComponent>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent", "i32"],
        },
        index_linkage!(Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>>,
    __registration__::__VEC_CONCAVE_COMPONENT_VTABLE,
    __registration__::__CONCAVE_COMPONENT_VTABLE,
    invalid,
    mutable
)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::ilen" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<line_segment_sketch::LineSegment<'eval>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_LINE_SEGMENT_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<line_segment_sketch::LineSegment>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::convexity::is_convex",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let line_segment_sketch: &line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
                let index: i32 = __arguments[1].downcast_i32();
                    line_segment_sketch::convexity::is_convex(line_segment_sketch, index).to_register()
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::convexity::is_convex as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::cyclic_slice" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &'eval Vec<line_segment_sketch::LineSegment<'eval>> = __arguments[0].downcast_eval_ref(&__registration__::__VEC_LINE_SEGMENT_VTABLE);
                let start: i32 = __arguments[1].downcast_i32();
                let end: i32 = __arguments[2].downcast_i32();
                    __Register::new_box(__this.cyclic_slice(start, end), &__registration__::__CYCLIC_SLICE_LINE_SEGMENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<line_segment_sketch::LineSegment>::cyclic_slice as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::push" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
                let element: line_segment_sketch::concave_component::ConcaveComponent<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<line_segment_sketch::concave_component::ConcaveComponent>::push as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let line_segment_sketch: &'eval line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
                let line_segments: __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CYCLIC_SLICE_LINE_SEGMENT_VTABLE);
                    __Register::new_box(line_segment_sketch::convex_component::ConvexCompoent::__call__(line_segment_sketch, line_segments), &__registration__::__CONVEX_COMPOENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::convex_component::ConvexCompoent::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent",
            field_ident: "line_segment_sketch",
        },
        eager_field_linkage!(line_segment_sketch::convex_component::ConvexCompoent<'eval>, __registration__::__CONVEX_COMPOENT_VTABLE, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, line_segment_sketch, box)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent",
            field_ident: "line_segments",
        },
        eager_field_linkage!(line_segment_sketch::convex_component::ConvexCompoent<'eval>, __registration__::__CONVEX_COMPOENT_VTABLE, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_VTABLE, line_segments, invalid)
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::line_segment_sketch::LineSegment::displacement" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &line_segment_sketch::LineSegment<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__LINE_SEGMENT_VTABLE);
                    __Register::new_box(__this.displacement(), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::LineSegment::displacement as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::rotation_direction_to" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    __this.rotation_direction_to(other).to_register()
                }
                __wrapper
            },
            opt_fp: Some(geom2d::Vector2d::rotation_direction_to as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::raw_contour::RawContour::displacement" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &raw_contour::RawContour<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                let start: i32 = __arguments[1].downcast_i32();
                let end: i32 = __arguments[2].downcast_i32();
                    __Register::new_box(__this.displacement(start, end), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(raw_contour::RawContour::displacement as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::cross" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    __this.cross(other).to_register()
                }
                __wrapper
            },
            opt_fp: Some(geom2d::Vector2d::cross as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "start",
        },
        eager_mut_field_linkage!(__std::slice::CyclicSlice<'eval, geom2d::Point2d>, __registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE, __registration__::__I32_VTABLE, start, direct)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "end",
        },
        eager_mut_field_linkage!(__std::slice::CyclicSlice<'eval, geom2d::Point2d>, __registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE, __registration__::__I32_VTABLE, end, direct)
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>", "i32"],
        },
        index_linkage!(__std::slice::CyclicSlice<'eval, geom2d::Point2d>,
    __registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE,
    __registration__::__POINT_2_D_VTABLE,
    invalid,
    immutable
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::LineSegment::new",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                let from: i32 = __arguments[1].downcast_i32();
                let to: i32 = __arguments[2].downcast_i32();
                    __Register::new_box(line_segment_sketch::LineSegment::new(ct, from, to), &__registration__::__LINE_SEGMENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::LineSegment::new as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::line_segment_sketch::LineSegment"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                                let __variadics =
                                    __arguments[0..]
                                        .iter_mut()
                                        .map(|v|v.downcast_move(&__registration__::__LINE_SEGMENT_VTABLE))
                                        .collect();
                    __Register::new_box(Vec::<line_segment_sketch::LineSegment>::__call__(__variadics), &__registration__::__VEC_LINE_SEGMENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<line_segment_sketch::LineSegment>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::line_segment_sketch::LineSegment", "i32"],
        },
        index_linkage!(Vec<line_segment_sketch::LineSegment<'eval>>,
    __registration__::__VEC_LINE_SEGMENT_VTABLE,
    __registration__::__LINE_SEGMENT_VTABLE,
    invalid,
    mutable
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::LineSegmentSketch::new",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                let r: f32 = __arguments[1].downcast_f32();
                    __Register::new_box(line_segment_sketch::LineSegmentSketch::new(ct, r), &__registration__::__LINE_SEGMENT_SKETCH_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::LineSegmentSketch::new as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::go_right",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let u: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                let r: f32 = __arguments[1].downcast_f32();
                    __Register::new_box(line_segment_sketch::go_right(u, r), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::go_right as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::go_left",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let u: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                let r: f32 = __arguments[1].downcast_f32();
                    __Register::new_box(line_segment_sketch::go_left(u, r), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::go_left as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::extend_end",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                let start: i32 = __arguments[1].downcast_i32();
                let r: f32 = __arguments[2].downcast_f32();
                    line_segment_sketch::extend_end(ct, start, r).to_register()
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::extend_end as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::norm" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    __this.norm().to_register()
                }
                __wrapper
            },
            opt_fp: Some(geom2d::Vector2d::norm as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::extend_start",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                let start0: i32 = __arguments[1].downcast_i32();
                let end: i32 = __arguments[2].downcast_i32();
                let r: f32 = __arguments[3].downcast_f32();
                    line_segment_sketch::extend_start(ct, start0, end, r).to_register()
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::extend_start as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::find_line_segments",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                let r: f32 = __arguments[1].downcast_f32();
                    __Register::new_box(line_segment_sketch::find_line_segments(ct, r), &__registration__::__VEC_LINE_SEGMENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(line_segment_sketch::find_line_segments as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::lastx" },
        method_elem_linkage!(Vec<line_segment_sketch::LineSegment<'eval>>, __registration__::__VEC_LINE_SEGMENT_VTABLE, __registration__::__LINE_SEGMENT_VTABLE, lastx)
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::dot" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    __this.dot(other).to_register()
                }
                __wrapper
            },
            opt_fp: Some(geom2d::Vector2d::dot as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Point2d::to" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &geom2d::Point2d = __arguments[0].downcast_temp_ref(&__registration__::__POINT_2_D_VTABLE);
                let other: &geom2d::Point2d = __arguments[1].downcast_temp_ref(&__registration__::__POINT_2_D_VTABLE);
                    __Register::new_box(__this.to(other), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(geom2d::Point2d::to as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::popx" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_LINE_SEGMENT_VTABLE);
                    __Register::new_box(__this.popx(), &__registration__::__LINE_SEGMENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<line_segment_sketch::LineSegment>::popx as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::push" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_LINE_SEGMENT_VTABLE);
                let element: line_segment_sketch::LineSegment<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__LINE_SEGMENT_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<line_segment_sketch::LineSegment>::push as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::firstx" },
        method_elem_linkage!(Vec<line_segment_sketch::LineSegment<'eval>>, __registration__::__VEC_LINE_SEGMENT_VTABLE, __registration__::__LINE_SEGMENT_VTABLE, firstx)
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::one::haha"
        },
        opt_feature_linkage!(one::haha, __registration__::__I32_VTABLE),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::fermi::FermiMatchResult"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let matches: Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE);
                let others: Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE);
                    __Register::new_box(fermi::FermiMatchResult::__call__(matches, others), &__registration__::__FERMI_MATCH_RESULT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(fermi::FermiMatchResult::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::fermi::FermiMatchResult",
            field_ident: "matches",
        },
        eager_field_linkage!(fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE, __registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE, matches, invalid)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "mnist_classifier::fermi::FermiMatchResult",
            field_ident: "others",
        },
        eager_field_linkage!(fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE, __registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE, others, invalid)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<?&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::ilen" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<Option<&line_segment_sketch::concave_component::ConcaveComponent>>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]?&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                                    let __variadics =
                                        __arguments[0..]
                                            .iter_mut()
                                            .map(|v|v.downcast_opt_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE))
                                            .collect();
                    __Register::new_box(Vec::<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent>>::__call__(__variadics), &__registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<Option<&line_segment_sketch::concave_component::ConcaveComponent>>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]?&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent", "i32"],
        },
        index_linkage!(Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>>,
    __registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE,
    __registration__::__CONCAVE_COMPONENT_VTABLE,
    invalid,
    mutable
)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::ilen" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<&line_segment_sketch::concave_component::ConcaveComponent>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                                    let __variadics =
                                        __arguments[0..]
                                            .iter_mut()
                                            .map(|v|v.downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE))
                                            .collect();
                    __Register::new_box(Vec::<&'eval line_segment_sketch::concave_component::ConcaveComponent>::__call__(__variadics), &__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<&line_segment_sketch::concave_component::ConcaveComponent>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent", "i32"],
        },
        index_linkage!(Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>,
    __registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE,
    __registration__::__CONCAVE_COMPONENT_VTABLE,
    invalid,
    mutable
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::fermi::fermi_match",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                let concave_components: &'eval Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_eval_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
                let templates: &Vec<fn(&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>)->Option<f32>> = __arguments[1].downcast_temp_ref(&__registration__::__VEC_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE);
                    __Register::new_box(fermi::fermi_match(concave_components, templates), &__registration__::__FERMI_MATCH_RESULT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(fermi::fermi_match as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<Fp<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, ?f32>>::ilen" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<fn(&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>)->Option<f32>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<for<'eval> fn(&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>)->Option<f32>>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]Fp<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, ?f32>"
        },

        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                                    let __variadics =
                                        __arguments[0..]
                                            .iter_mut()
                                            .map(|v| {
                                                std::mem::transmute(v.downcast_temp_ref::<__VirtualFunction>(&__registration__::__FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE).fp())
                                            })
                                            .collect();
                    __Register::new_box(Vec::<fn(&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>)->Option<f32>>::__call__(__variadics), &__registration__::__VEC_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<for<'eval> fn(&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>)->Option<f32>>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]Fp<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, ?f32>", "i32"],
        },
        index_linkage!(Vec<fn(&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>)->Option<f32>>,
    __registration__::__VEC_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE,
    __registration__::__FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE,
    invalid,
    mutable
)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::collect_refs" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &'eval Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_eval_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
                    __Register::new_box(__this.collect_refs(), &__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<line_segment_sketch::concave_component::ConcaveComponent>::collect_refs as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::pop_with_largest_opt_f32" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE);
                let f: fn(&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>)->Option<f32> = std::mem::transmute(__arguments[1]
        .downcast_temp_ref::<__VirtualFunction>(&__registration__::__VIRTUAL_FUNCTION_VTABLE)
        .fp());
                    __Register::new_opt_box(__this.pop_with_largest_opt_f32_copyable(f), &__registration__::__CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<&line_segment_sketch::concave_component::ConcaveComponent>::pop_with_largest_opt_f32_copyable as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<?&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::push" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE);
                let element: Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<Option<&line_segment_sketch::concave_component::ConcaveComponent>>::push as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::cyclic_slice" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &'eval Vec<geom2d::Point2d> = __arguments[0].downcast_eval_ref(&__registration__::__VEC_POINT_2_D_VTABLE);
                let start: i32 = __arguments[1].downcast_i32();
                let end: i32 = __arguments[2].downcast_i32();
                    __Register::new_box(__this.cyclic_slice(start, end), &__registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(Vec::<geom2d::Point2d>::cyclic_slice as *const ()),
        }),
    ),
];
