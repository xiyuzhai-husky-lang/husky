use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::connected_component::ConnectedComponentDistribution"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let row_start: i32 = __arguments[0].downcast_i32();
                    let row_end: i32 = __arguments[1].downcast_i32();
                    let upper_mass: i32 = __arguments[2].downcast_i32();
                    let lower_mass: i32 = __arguments[3].downcast_i32();
                    __Register::new_box::<connected_component::ConnectedComponentDistribution>(connected_component::ConnectedComponentDistribution::__call__(row_start, row_end, upper_mass, lower_mass), &__registration__::__CONNECTED_COMPONENT_DISTRIBUTION_VTABLE)
                }
                __wrapper
            },
            some base connected_component::ConnectedComponentDistribution::__call__ as fn(i32, i32, i32, i32) -> connected_component::ConnectedComponentDistribution
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponentDistribution",
            field_ident: "row_start",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            connected_component::ConnectedComponentDistribution, __registration__::__CONNECTED_COMPONENT_DISTRIBUTION_VTABLE, i32, __registration__::__I32_VTABLE,
            row_start
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponentDistribution",
            field_ident: "row_end",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            connected_component::ConnectedComponentDistribution, __registration__::__CONNECTED_COMPONENT_DISTRIBUTION_VTABLE, i32, __registration__::__I32_VTABLE,
            row_end
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponentDistribution",
            field_ident: "upper_mass",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            connected_component::ConnectedComponentDistribution, __registration__::__CONNECTED_COMPONENT_DISTRIBUTION_VTABLE, i32, __registration__::__I32_VTABLE,
            upper_mass
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponentDistribution",
            field_ident: "lower_mass",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            connected_component::ConnectedComponentDistribution, __registration__::__CONNECTED_COMPONENT_DISTRIBUTION_VTABLE, i32, __registration__::__I32_VTABLE,
            lower_mass
        )
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::connected_component::EffHoles"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let matches: Vec<Option<&'eval raw_contour::RawContour<'eval>>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__VEC_OPTION_REF_RAW_CONTOUR_VTABLE);
                    __Register::new_box::<connected_component::EffHoles<'eval>>(connected_component::EffHoles::__call__(matches), &__registration__::__EFF_HOLES_VTABLE)
                }
                __wrapper
            },
            some base connected_component::EffHoles::__call__ as fn(Vec<Option<&'static raw_contour::RawContour<'static>>>) -> connected_component::EffHoles<'static>
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::EffHoles",
            field_ident: "matches",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            connected_component::EffHoles<'eval>, __registration__::__EFF_HOLES_VTABLE, Vec<Option<&'eval raw_contour::RawContour<'eval>>>, __registration__::__VEC_OPTION_REF_RAW_CONTOUR_VTABLE,
            matches
        )
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<?&mnist_classifier::raw_contour::RawContour>::ilen" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<Option<&'eval raw_contour::RawContour<'eval>>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_OPTION_REF_RAW_CONTOUR_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<Option<&raw_contour::RawContour>>::ilen as fn(&'static Vec<Option<&'static raw_contour::RawContour<'static>>>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::raw_contour::RawContour"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval connected_component::ConnectedComponent = __arguments[0].downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
                    let points: Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__VEC_POINT_2_D_VTABLE);
                    __Register::new_box::<raw_contour::RawContour<'eval>>(raw_contour::RawContour::__call__(cc, points), &__registration__::__RAW_CONTOUR_VTABLE)
                }
                __wrapper
            },
            some base raw_contour::RawContour::__call__ as fn(&'static connected_component::ConnectedComponent, Vec<geom2d::Point2d>) -> raw_contour::RawContour<'static>
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "cc",
        },
        eager_field_linkage!(
            immutable,
            EvalRef,
            BoxNonCopyable,
            raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE, connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE,
            cc
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "points",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE, Vec<geom2d::Point2d>, __registration__::__VEC_POINT_2_D_VTABLE,
            points
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "line_segment_sketch",
        },
        lazy_field_linkage!(raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE, line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, line_segment_sketch)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "bounding_box",
        },
        lazy_field_linkage!(raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE, geom2d::BoundingBox, __registration__::__BOUNDING_BOX_VTABLE, bounding_box)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "relative_bounding_box",
        },
        lazy_field_linkage!(raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE, geom2d::RelativeBoundingBox, __registration__::__RELATIVE_BOUNDING_BOX_VTABLE, relative_bounding_box)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "contour_len",
        },
        lazy_field_linkage!(raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE, f32, __registration__::__F32_VTABLE, contour_len)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]?&mnist_classifier::raw_contour::RawContour"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_opt_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE))
                            .collect();
                    __Register::new_box::<Vec<Option<&'eval raw_contour::RawContour<'eval>>>>(Vec::<Option<&'eval raw_contour::RawContour>>::__call__(__variadics), &__registration__::__VEC_OPTION_REF_RAW_CONTOUR_VTABLE)
                }
                __wrapper
            },
            some base Vec::<Option<&raw_contour::RawContour>>::__call__ as fn(Vec<Option<&'static raw_contour::RawContour<'static>>>) -> Vec<Option<&'static raw_contour::RawContour<'static>>>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]?&mnist_classifier::raw_contour::RawContour", "i32"],
        },
        index_linkage!(
            mutable,
            OptionalEvalRef,
            BoxNonCopyable,Vec<Option<&'eval raw_contour::RawContour<'eval>>>,
            __registration__::__VEC_OPTION_REF_RAW_CONTOUR_VTABLE,
            raw_contour::RawContour<'eval>,
            __registration__::__RAW_CONTOUR_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::connected_component::hole_tmpl",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                    connected_component::hole_tmpl(ct, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx connected_component::hole_tmpl as fn(&'static raw_contour::RawContour<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::connected_component::ConnectedComponent"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let mask: domains::ml::datasets::cv::mnist::BinaryImage28 = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__BINARY_IMAGE_28_VTABLE);
                    __Register::new_box::<connected_component::ConnectedComponent>(connected_component::ConnectedComponent::__call__(mask), &__registration__::__CONNECTED_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some base connected_component::ConnectedComponent::__call__ as fn(domains::ml::datasets::cv::mnist::BinaryImage28) -> connected_component::ConnectedComponent
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "mask",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, domains::ml::datasets::cv::mnist::BinaryImage28, __registration__::__BINARY_IMAGE_28_VTABLE,
            mask
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "raw_contours",
        },
        lazy_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, Vec<raw_contour::RawContour<'eval>>, __registration__::__VEC_RAW_CONTOUR_VTABLE, raw_contours)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "eff_holes",
        },
        lazy_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, connected_component::EffHoles<'eval>, __registration__::__EFF_HOLES_VTABLE, eff_holes)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "max_hole_ilen",
        },
        lazy_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, f32, __registration__::__F32_VTABLE, max_hole_ilen)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "max_row_span",
        },
        lazy_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, f32, __registration__::__F32_VTABLE, max_row_span)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "row_span_sum",
        },
        lazy_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, f32, __registration__::__F32_VTABLE, row_span_sum)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "distribution",
        },
        lazy_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, connected_component::ConnectedComponentDistribution, __registration__::__CONNECTED_COMPONENT_DISTRIBUTION_VTABLE, distribution)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "upper_mass",
        },
        lazy_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, f32, __registration__::__F32_VTABLE, upper_mass)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "lower_mass",
        },
        lazy_field_linkage!(connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE, f32, __registration__::__F32_VTABLE, lower_mass)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::raw_contour::RawContour>::ilen" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<raw_contour::RawContour<'eval>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_RAW_CONTOUR_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<raw_contour::RawContour>::ilen as fn(&'static Vec<raw_contour::RawContour<'static>>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::raw_contour::RawContour"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_move(&__registration__::__RAW_CONTOUR_VTABLE))
                            .collect();
                    __Register::new_box::<Vec<raw_contour::RawContour<'eval>>>(Vec::<raw_contour::RawContour>::__call__(__variadics), &__registration__::__VEC_RAW_CONTOUR_VTABLE)
                }
                __wrapper
            },
            some base Vec::<raw_contour::RawContour>::__call__ as fn(Vec<raw_contour::RawContour<'static>>) -> Vec<raw_contour::RawContour<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::raw_contour::RawContour", "i32"],
        },
        index_linkage!(
            mutable,
            Intrinsic,
            BoxNonCopyable,Vec<raw_contour::RawContour<'eval>>,
            __registration__::__VEC_RAW_CONTOUR_VTABLE,
            raw_contour::RawContour<'eval>,
            __registration__::__RAW_CONTOUR_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::connected_component::horizontal_extend",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let a: u32 = __arguments[0].downcast_b32();
                    let x: u32 = __arguments[1].downcast_b32();
                    connected_component::horizontal_extend(a, x).to_register()
                }
                __wrapper
            },
            some base connected_component::horizontal_extend as fn(u32, u32) -> u32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::connected_component::find_connected_components",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let img: &domains::ml::datasets::cv::mnist::BinaryImage28 = __arguments[0].downcast_temp_ref(&__registration__::__BINARY_IMAGE_28_VTABLE);
                    __Register::new_box::<Vec<connected_component::ConnectedComponent>>(connected_component::find_connected_components(img), &__registration__::__VEC_CONNECTED_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some base connected_component::find_connected_components as fn(&'static domains::ml::datasets::cv::mnist::BinaryImage28) -> Vec<connected_component::ConnectedComponent>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::connected_component::ConnectedComponent>::ilen" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<connected_component::ConnectedComponent> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_CONNECTED_COMPONENT_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<connected_component::ConnectedComponent>::ilen as fn(&'static Vec<connected_component::ConnectedComponent>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::connected_component::ConnectedComponent"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_move(&__registration__::__CONNECTED_COMPONENT_VTABLE))
                            .collect();
                    __Register::new_box::<Vec<connected_component::ConnectedComponent>>(Vec::<connected_component::ConnectedComponent>::__call__(__variadics), &__registration__::__VEC_CONNECTED_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some base Vec::<connected_component::ConnectedComponent>::__call__ as fn(Vec<connected_component::ConnectedComponent>) -> Vec<connected_component::ConnectedComponent>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::connected_component::ConnectedComponent", "i32"],
        },
        index_linkage!(
            mutable,
            Intrinsic,
            BoxNonCopyable,Vec<connected_component::ConnectedComponent>,
            __registration__::__VEC_CONNECTED_COMPONENT_VTABLE,
            connected_component::ConnectedComponent,
            __registration__::__CONNECTED_COMPONENT_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::connected_component::ConnectedComponent>::push" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<connected_component::ConnectedComponent> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_CONNECTED_COMPONENT_VTABLE);
                    let element: connected_component::ConnectedComponent = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CONNECTED_COMPONENT_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            some base Vec::<connected_component::ConnectedComponent>::push as fn(&'static mut Vec<connected_component::ConnectedComponent>, connected_component::ConnectedComponent) -> ()
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::ilen" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<geom2d::Point2d> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_POINT_2_D_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<geom2d::Point2d>::ilen as fn(&'static Vec<geom2d::Point2d>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::Point2d"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let x: f32 = __arguments[0].downcast_f32();
                    let y: f32 = __arguments[1].downcast_f32();
                    __Register::new_box::<geom2d::Point2d>(geom2d::Point2d::__call__(x, y), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base geom2d::Point2d::__call__ as fn(f32, f32) -> geom2d::Point2d
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::Point2d",
            field_ident: "x",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            geom2d::Point2d, __registration__::__POINT_2_D_VTABLE, f32, __registration__::__F32_VTABLE,
            x
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::Point2d",
            field_ident: "y",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            geom2d::Point2d, __registration__::__POINT_2_D_VTABLE, f32, __registration__::__F32_VTABLE,
            y
        )
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::geom2d::Point2d"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_move(&__registration__::__POINT_2_D_VTABLE))
                            .collect();
                    __Register::new_box::<Vec<geom2d::Point2d>>(Vec::<geom2d::Point2d>::__call__(__variadics), &__registration__::__VEC_POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base Vec::<geom2d::Point2d>::__call__ as fn(Vec<geom2d::Point2d>) -> Vec<geom2d::Point2d>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::geom2d::Point2d", "i32"],
        },
        index_linkage!(
            mutable,
            Intrinsic,
            BoxNonCopyable,Vec<geom2d::Point2d>,
            __registration__::__VEC_POINT_2_D_VTABLE,
            geom2d::Point2d,
            __registration__::__POINT_2_D_VTABLE
)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let contour: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                    let strokes: Vec<line_segment_sketch::LineSegmentStroke<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE);
                    __Register::new_box::<line_segment_sketch::LineSegmentSketch<'eval>>(line_segment_sketch::LineSegmentSketch::__call__(contour, strokes), &__registration__::__LINE_SEGMENT_SKETCH_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::LineSegmentSketch::__call__ as fn(&'static raw_contour::RawContour<'static>, Vec<line_segment_sketch::LineSegmentStroke<'static>>) -> line_segment_sketch::LineSegmentSketch<'static>
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "contour",
        },
        eager_field_linkage!(
            immutable,
            EvalRef,
            BoxNonCopyable,
            line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE,
            contour
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "strokes",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, Vec<line_segment_sketch::LineSegmentStroke<'eval>>, __registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE,
            strokes
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "concave_components",
        },
        lazy_field_linkage!(line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>>, __registration__::__VEC_CONCAVE_COMPONENT_VTABLE, concave_components)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "bounding_box",
        },
        lazy_field_linkage!(line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE, geom2d::BoundingBox, __registration__::__BOUNDING_BOX_VTABLE, bounding_box)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::BoundingBox"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let xrange: geom2d::ClosedRange = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__CLOSED_RANGE_VTABLE);
                    let yrange: geom2d::ClosedRange = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CLOSED_RANGE_VTABLE);
                    __Register::new_box::<geom2d::BoundingBox>(geom2d::BoundingBox::__call__(xrange, yrange), &__registration__::__BOUNDING_BOX_VTABLE)
                }
                __wrapper
            },
            some base geom2d::BoundingBox::__call__ as fn(geom2d::ClosedRange, geom2d::ClosedRange) -> geom2d::BoundingBox
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::BoundingBox",
            field_ident: "xrange",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            geom2d::BoundingBox, __registration__::__BOUNDING_BOX_VTABLE, geom2d::ClosedRange, __registration__::__CLOSED_RANGE_VTABLE,
            xrange
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::BoundingBox",
            field_ident: "yrange",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            geom2d::BoundingBox, __registration__::__BOUNDING_BOX_VTABLE, geom2d::ClosedRange, __registration__::__CLOSED_RANGE_VTABLE,
            yrange
        )
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::RelativeBoundingBox"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let xrange: geom2d::ClosedRange = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__CLOSED_RANGE_VTABLE);
                    let yrange: geom2d::ClosedRange = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CLOSED_RANGE_VTABLE);
                    __Register::new_box::<geom2d::RelativeBoundingBox>(geom2d::RelativeBoundingBox::__call__(xrange, yrange), &__registration__::__RELATIVE_BOUNDING_BOX_VTABLE)
                }
                __wrapper
            },
            some base geom2d::RelativeBoundingBox::__call__ as fn(geom2d::ClosedRange, geom2d::ClosedRange) -> geom2d::RelativeBoundingBox
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::RelativeBoundingBox",
            field_ident: "xrange",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            geom2d::RelativeBoundingBox, __registration__::__RELATIVE_BOUNDING_BOX_VTABLE, geom2d::ClosedRange, __registration__::__CLOSED_RANGE_VTABLE,
            xrange
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::RelativeBoundingBox",
            field_ident: "yrange",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            geom2d::RelativeBoundingBox, __registration__::__RELATIVE_BOUNDING_BOX_VTABLE, geom2d::ClosedRange, __registration__::__CLOSED_RANGE_VTABLE,
            yrange
        )
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_pixel_pair",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let row: u32 = __arguments[0].downcast_b32();
                    let j: i32 = __arguments[1].downcast_i32();
                    raw_contour::get_pixel_pair(row, j).to_register()
                }
                __wrapper
            },
            some base raw_contour::get_pixel_pair as fn(u32, i32) -> u32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_pixel_to_the_left",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let row: u32 = __arguments[0].downcast_b32();
                    let j: i32 = __arguments[1].downcast_i32();
                    raw_contour::get_pixel_to_the_left(row, j).to_register()
                }
                __wrapper
            },
            some base raw_contour::get_pixel_to_the_left as fn(u32, i32) -> u32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_pixel_to_the_right",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let row: u32 = __arguments[0].downcast_b32();
                    let j: i32 = __arguments[1].downcast_i32();
                    raw_contour::get_pixel_to_the_right(row, j).to_register()
                }
                __wrapper
            },
            some base raw_contour::get_pixel_to_the_right as fn(u32, i32) -> u32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_inward_direction",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let row_above: u32 = __arguments[0].downcast_b32();
                    let row_below: u32 = __arguments[1].downcast_b32();
                    let j: i32 = __arguments[2].downcast_i32();
                    __Register::new_box::<raw_contour::Direction>(raw_contour::get_inward_direction(row_above, row_below, j), &__registration__::__DIRECTION_VTABLE)
                }
                __wrapper
            },
            some base raw_contour::get_inward_direction as fn(u32, u32, i32) -> raw_contour::Direction
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_angle_change",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let inward: raw_contour::Direction = __arguments[0].downcast_temp_ref::<__VirtualEnum>(&__registration__::__VIRTUAL_ENUM_VTABLE).kind_idx.into();
                    let outward: raw_contour::Direction = __arguments[1].downcast_temp_ref::<__VirtualEnum>(&__registration__::__VIRTUAL_ENUM_VTABLE).kind_idx.into();
                    raw_contour::get_angle_change(inward, outward).to_register()
                }
                __wrapper
            },
            some base raw_contour::get_angle_change as fn(raw_contour::Direction, raw_contour::Direction) -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_outward_direction",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let row_above: u32 = __arguments[0].downcast_b32();
                    let row_below: u32 = __arguments[1].downcast_b32();
                    let j: i32 = __arguments[2].downcast_i32();
                    let inward_direction: raw_contour::Direction = __arguments[3].downcast_temp_ref::<__VirtualEnum>(&__registration__::__VIRTUAL_ENUM_VTABLE).kind_idx.into();
                    __Register::new_box::<raw_contour::Direction>(raw_contour::get_outward_direction(row_above, row_below, j, inward_direction), &__registration__::__DIRECTION_VTABLE)
                }
                __wrapper
            },
            some base raw_contour::get_outward_direction as fn(u32, u32, i32, raw_contour::Direction) -> raw_contour::Direction
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::raw_contour::StreakCache"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let prev1: i32 = __arguments[0].downcast_i32();
                    let prev2: i32 = __arguments[1].downcast_i32();
                    __Register::new_box::<raw_contour::StreakCache>(raw_contour::StreakCache::__call__(prev1, prev2), &__registration__::__STREAK_CACHE_VTABLE)
                }
                __wrapper
            },
            some base raw_contour::StreakCache::__call__ as fn(i32, i32) -> raw_contour::StreakCache
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::raw_contour::StreakCache",
            field_ident: "prev1",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            raw_contour::StreakCache, __registration__::__STREAK_CACHE_VTABLE, i32, __registration__::__I32_VTABLE,
            prev1
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::raw_contour::StreakCache",
            field_ident: "prev2",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            raw_contour::StreakCache, __registration__::__STREAK_CACHE_VTABLE, i32, __registration__::__I32_VTABLE,
            prev2
        )
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::get_concave_middle_point",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let points: &Vec<geom2d::Point2d> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_POINT_2_D_VTABLE);
                    __Register::new_box::<geom2d::Point2d>(raw_contour::get_concave_middle_point(points), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base raw_contour::get_concave_middle_point as fn(&'static Vec<geom2d::Point2d>) -> geom2d::Point2d
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::raw_contour::find_raw_contours",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval connected_component::ConnectedComponent = __arguments[0].downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
                    __Register::new_box::<Vec<raw_contour::RawContour<'eval>>>(raw_contour::find_raw_contours(cc), &__registration__::__VEC_RAW_CONTOUR_VTABLE)
                }
                __wrapper
            },
            some base raw_contour::find_raw_contours as fn(&'static connected_component::ConnectedComponent) -> Vec<raw_contour::RawContour<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::lastx" },
        method_elem_linkage!(Vec<geom2d::Point2d>, __registration__::__VEC_POINT_2_D_VTABLE, __registration__::__POINT_2_D_VTABLE, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::geom2d::Point2d::from_i_shift28",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let i: i32 = __arguments[0].downcast_i32();
                    let shift: i32 = __arguments[1].downcast_i32();
                    __Register::new_box::<geom2d::Point2d>(geom2d::Point2d::from_i_shift28(i, shift), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base geom2d::Point2d::from_i_shift28 as fn(i32, i32) -> geom2d::Point2d
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::push" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_POINT_2_D_VTABLE);
                    let element: geom2d::Point2d = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__POINT_2_D_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            some base Vec::<geom2d::Point2d>::push as fn(&'static mut Vec<geom2d::Point2d>, geom2d::Point2d) -> ()
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::popx" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_POINT_2_D_VTABLE);
                    __Register::new_box::<geom2d::Point2d>(__this.popx(), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base Vec::<geom2d::Point2d>::popx as fn(&'static mut Vec<geom2d::Point2d>) -> geom2d::Point2d
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::raw_contour::RawContour>::push" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<raw_contour::RawContour<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_RAW_CONTOUR_VTABLE);
                    let element: raw_contour::RawContour<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__RAW_CONTOUR_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            some base Vec::<raw_contour::RawContour>::push as fn(&'static mut Vec<raw_contour::RawContour<'static>>, raw_contour::RawContour<'static>) -> ()
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::RelativePoint2d"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let x: f32 = __arguments[0].downcast_f32();
                    let y: f32 = __arguments[1].downcast_f32();
                    __Register::new_box::<geom2d::RelativePoint2d>(geom2d::RelativePoint2d::__call__(x, y), &__registration__::__RELATIVE_POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base geom2d::RelativePoint2d::__call__ as fn(f32, f32) -> geom2d::RelativePoint2d
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::RelativePoint2d",
            field_ident: "x",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            geom2d::RelativePoint2d, __registration__::__RELATIVE_POINT_2_D_VTABLE, f32, __registration__::__F32_VTABLE,
            x
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::RelativePoint2d",
            field_ident: "y",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            geom2d::RelativePoint2d, __registration__::__RELATIVE_POINT_2_D_VTABLE, f32, __registration__::__F32_VTABLE,
            y
        )
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::Vector2d"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let x: f32 = __arguments[0].downcast_f32();
                    let y: f32 = __arguments[1].downcast_f32();
                    __Register::new_box::<geom2d::Vector2d>(geom2d::Vector2d::__call__(x, y), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            some base geom2d::Vector2d::__call__ as fn(f32, f32) -> geom2d::Vector2d
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::Vector2d",
            field_ident: "x",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            geom2d::Vector2d, __registration__::__VECTOR_2_D_VTABLE, f32, __registration__::__F32_VTABLE,
            x
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::Vector2d",
            field_ident: "y",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            geom2d::Vector2d, __registration__::__VECTOR_2_D_VTABLE, f32, __registration__::__F32_VTABLE,
            y
        )
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::ClosedRange"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let min: f32 = __arguments[0].downcast_f32();
                    let max: f32 = __arguments[1].downcast_f32();
                    __Register::new_box::<geom2d::ClosedRange>(geom2d::ClosedRange::__call__(min, max), &__registration__::__CLOSED_RANGE_VTABLE)
                }
                __wrapper
            },
            some base geom2d::ClosedRange::__call__ as fn(f32, f32) -> geom2d::ClosedRange
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::ClosedRange",
            field_ident: "min",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            geom2d::ClosedRange, __registration__::__CLOSED_RANGE_VTABLE, f32, __registration__::__F32_VTABLE,
            min
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::geom2d::ClosedRange",
            field_ident: "max",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            geom2d::ClosedRange, __registration__::__CLOSED_RANGE_VTABLE, f32, __registration__::__F32_VTABLE,
            max
        )
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let line_segment_sketch: &'eval line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
                    let strokes: __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE);
                    __Register::new_box::<line_segment_sketch::concave_component::ConcaveComponent<'eval>>(line_segment_sketch::concave_component::ConcaveComponent::__call__(line_segment_sketch, strokes), &__registration__::__CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::concave_component::ConcaveComponent::__call__ as fn(&'static line_segment_sketch::LineSegmentSketch<'static>, __std::slice::CyclicSlice<'static, line_segment_sketch::LineSegmentStroke<'static>>) -> line_segment_sketch::concave_component::ConcaveComponent<'static>
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "line_segment_sketch",
        },
        eager_field_linkage!(
            immutable,
            EvalRef,
            BoxNonCopyable,
            line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE,
            line_segment_sketch
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "strokes",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>>, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE,
            strokes
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "norm",
        },
        lazy_field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, f32, __registration__::__F32_VTABLE, norm)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "rel_norm",
        },
        lazy_field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, f32, __registration__::__F32_VTABLE, rel_norm)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "hausdorff_norm",
        },
        lazy_field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, f32, __registration__::__F32_VTABLE, hausdorff_norm)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "angle_change",
        },
        lazy_field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, f32, __registration__::__F32_VTABLE, angle_change)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "bounding_box",
        },
        lazy_field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, geom2d::BoundingBox, __registration__::__BOUNDING_BOX_VTABLE, bounding_box)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "relative_bounding_box",
        },
        lazy_field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, __registration__::__CONCAVE_COMPONENT_VTABLE, geom2d::RelativeBoundingBox, __registration__::__RELATIVE_BOUNDING_BOX_VTABLE, relative_bounding_box)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::LineSegmentStroke"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let points: __std::slice::CyclicSlice<'eval, geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE);
                    __Register::new_box::<line_segment_sketch::LineSegmentStroke<'eval>>(line_segment_sketch::LineSegmentStroke::__call__(points), &__registration__::__LINE_SEGMENT_STROKE_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::LineSegmentStroke::__call__ as fn(__std::slice::CyclicSlice<'static, geom2d::Point2d>) -> line_segment_sketch::LineSegmentStroke<'static>
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentStroke",
            field_ident: "points",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            line_segment_sketch::LineSegmentStroke<'eval>, __registration__::__LINE_SEGMENT_STROKE_VTABLE, __std::slice::CyclicSlice<'eval, geom2d::Point2d>, __registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE,
            points
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentStroke",
            field_ident: "start",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            line_segment_sketch::LineSegmentStroke<'eval>, __registration__::__LINE_SEGMENT_STROKE_VTABLE, geom2d::Point2d, __registration__::__POINT_2_D_VTABLE,
            start
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentStroke",
            field_ident: "end",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            line_segment_sketch::LineSegmentStroke<'eval>, __registration__::__LINE_SEGMENT_STROKE_VTABLE, geom2d::Point2d, __registration__::__POINT_2_D_VTABLE,
            end
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegmentStroke>",
            field_ident: "start",
        },
        eager_field_linkage!(
            mutable,
            Intrinsic,
            Direct,
            __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>>, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE, i32, __registration__::__I32_VTABLE,
            start
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegmentStroke>",
            field_ident: "end",
        },
        eager_field_linkage!(
            mutable,
            Intrinsic,
            Direct,
            __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>>, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE, i32, __registration__::__I32_VTABLE,
            end
        )
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegmentStroke>", "i32"],
        },
        index_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>>,
            __registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE,
            line_segment_sketch::LineSegmentStroke<'eval>,
            __registration__::__LINE_SEGMENT_STROKE_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::concave_component::find_concave_components",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let line_segment_sketch: &'eval line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
                    __Register::new_box::<Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>>>(line_segment_sketch::concave_component::find_concave_components(line_segment_sketch), &__registration__::__VEC_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::concave_component::find_concave_components as fn(&'static line_segment_sketch::LineSegmentSketch<'static>) -> Vec<line_segment_sketch::concave_component::ConcaveComponent<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::ilen" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<line_segment_sketch::concave_component::ConcaveComponent>::ilen as fn(&'static Vec<line_segment_sketch::concave_component::ConcaveComponent<'static>>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_move(&__registration__::__CONCAVE_COMPONENT_VTABLE))
                            .collect();
                    __Register::new_box::<Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>>>(Vec::<line_segment_sketch::concave_component::ConcaveComponent>::__call__(__variadics), &__registration__::__VEC_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some base Vec::<line_segment_sketch::concave_component::ConcaveComponent>::__call__ as fn(Vec<line_segment_sketch::concave_component::ConcaveComponent<'static>>) -> Vec<line_segment_sketch::concave_component::ConcaveComponent<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent", "i32"],
        },
        index_linkage!(
            mutable,
            Intrinsic,
            BoxNonCopyable,Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>>,
            __registration__::__VEC_CONCAVE_COMPONENT_VTABLE,
            line_segment_sketch::concave_component::ConcaveComponent<'eval>,
            __registration__::__CONCAVE_COMPONENT_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>::ilen" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<line_segment_sketch::LineSegmentStroke<'eval>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<line_segment_sketch::LineSegmentStroke>::ilen as fn(&'static Vec<line_segment_sketch::LineSegmentStroke<'static>>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::convexity::is_convex",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let line_segment_sketch: &line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
                    let index: i32 = __arguments[1].downcast_i32();
                    line_segment_sketch::convexity::is_convex(line_segment_sketch, index).to_register()
                }
                __wrapper
            },
            some base line_segment_sketch::convexity::is_convex as fn(&'static line_segment_sketch::LineSegmentSketch<'static>, i32) -> bool
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>::cyclic_slice" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &'eval Vec<line_segment_sketch::LineSegmentStroke<'eval>> = __arguments[0].downcast_eval_ref(&__registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE);
                    let start: i32 = __arguments[1].downcast_i32();
                    let end: i32 = __arguments[2].downcast_i32();
                    __Register::new_box::<__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>>>(__this.cyclic_slice(start, end), &__registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE)
                }
                __wrapper
            },
            some base Vec::<line_segment_sketch::LineSegmentStroke>::cyclic_slice as fn(&'static Vec<line_segment_sketch::LineSegmentStroke<'static>>, i32, i32) -> __std::slice::CyclicSlice<'static, line_segment_sketch::LineSegmentStroke<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::push" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
                    let element: line_segment_sketch::concave_component::ConcaveComponent<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            some base Vec::<line_segment_sketch::concave_component::ConcaveComponent>::push as fn(&'static mut Vec<line_segment_sketch::concave_component::ConcaveComponent<'static>>, line_segment_sketch::concave_component::ConcaveComponent<'static>) -> ()
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let line_segment_sketch: &'eval line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
                    let line_segments: __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE);
                    __Register::new_box::<line_segment_sketch::convex_component::ConvexCompoent<'eval>>(line_segment_sketch::convex_component::ConvexCompoent::__call__(line_segment_sketch, line_segments), &__registration__::__CONVEX_COMPOENT_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::convex_component::ConvexCompoent::__call__ as fn(&'static line_segment_sketch::LineSegmentSketch<'static>, __std::slice::CyclicSlice<'static, line_segment_sketch::LineSegmentStroke<'static>>) -> line_segment_sketch::convex_component::ConvexCompoent<'static>
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent",
            field_ident: "line_segment_sketch",
        },
        eager_field_linkage!(
            immutable,
            EvalRef,
            BoxNonCopyable,
            line_segment_sketch::convex_component::ConvexCompoent<'eval>, __registration__::__CONVEX_COMPOENT_VTABLE, line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE,
            line_segment_sketch
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent",
            field_ident: "line_segments",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            line_segment_sketch::convex_component::ConvexCompoent<'eval>, __registration__::__CONVEX_COMPOENT_VTABLE, __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>>, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE,
            line_segments
        )
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::line_segment_sketch::LineSegmentStroke::displacement" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &line_segment_sketch::LineSegmentStroke<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__LINE_SEGMENT_STROKE_VTABLE);
                    __Register::new_box::<geom2d::Vector2d>(__this.displacement(), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::LineSegmentStroke::displacement as fn(&'static line_segment_sketch::LineSegmentStroke<'static>) -> geom2d::Vector2d
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::rotation_direction_to" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    __this.rotation_direction_to(other).to_register()
                }
                __wrapper
            },
            some base geom2d::Vector2d::rotation_direction_to as fn(&'static geom2d::Vector2d, &'static geom2d::Vector2d) -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::raw_contour::RawContour::displacement" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &raw_contour::RawContour<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                    let start: i32 = __arguments[1].downcast_i32();
                    let end: i32 = __arguments[2].downcast_i32();
                    __Register::new_box::<geom2d::Vector2d>(__this.displacement(start, end), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            some base raw_contour::RawContour::displacement as fn(&'static raw_contour::RawContour<'static>, i32, i32) -> geom2d::Vector2d
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::cross" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    __this.cross(other).to_register()
                }
                __wrapper
            },
            some base geom2d::Vector2d::cross as fn(&'static geom2d::Vector2d, &'static geom2d::Vector2d) -> f32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::line_segment::LineSegment"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let start: geom2d::Point2d = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__POINT_2_D_VTABLE);
                    let end: geom2d::Point2d = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__POINT_2_D_VTABLE);
                    __Register::new_box::<line_segment_sketch::line_segment::LineSegment>(line_segment_sketch::line_segment::LineSegment::__call__(start, end), &__registration__::__LINE_SEGMENT_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::line_segment::LineSegment::__call__ as fn(geom2d::Point2d, geom2d::Point2d) -> line_segment_sketch::line_segment::LineSegment
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::line_segment::LineSegment",
            field_ident: "start",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            line_segment_sketch::line_segment::LineSegment, __registration__::__LINE_SEGMENT_VTABLE, geom2d::Point2d, __registration__::__POINT_2_D_VTABLE,
            start
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::line_segment_sketch::line_segment::LineSegment",
            field_ident: "end",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            line_segment_sketch::line_segment::LineSegment, __registration__::__LINE_SEGMENT_VTABLE, geom2d::Point2d, __registration__::__POINT_2_D_VTABLE,
            end
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "start",
        },
        eager_field_linkage!(
            mutable,
            Intrinsic,
            Direct,
            __std::slice::CyclicSlice<'eval, geom2d::Point2d>, __registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE, i32, __registration__::__I32_VTABLE,
            start
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "end",
        },
        eager_field_linkage!(
            mutable,
            Intrinsic,
            Direct,
            __std::slice::CyclicSlice<'eval, geom2d::Point2d>, __registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE, i32, __registration__::__I32_VTABLE,
            end
        )
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>", "i32"],
        },
        index_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,__std::slice::CyclicSlice<'eval, geom2d::Point2d>,
            __registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE,
            geom2d::Point2d,
            __registration__::__POINT_2_D_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::LineSegmentStroke::new",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                    let from: i32 = __arguments[1].downcast_i32();
                    let to: i32 = __arguments[2].downcast_i32();
                    __Register::new_box::<line_segment_sketch::LineSegmentStroke<'eval>>(line_segment_sketch::LineSegmentStroke::new(ct, from, to), &__registration__::__LINE_SEGMENT_STROKE_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::LineSegmentStroke::new as fn(&'static raw_contour::RawContour<'static>, i32, i32) -> line_segment_sketch::LineSegmentStroke<'static>
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]mnist_classifier::line_segment_sketch::LineSegmentStroke"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_move(&__registration__::__LINE_SEGMENT_STROKE_VTABLE))
                            .collect();
                    __Register::new_box::<Vec<line_segment_sketch::LineSegmentStroke<'eval>>>(Vec::<line_segment_sketch::LineSegmentStroke>::__call__(__variadics), &__registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE)
                }
                __wrapper
            },
            some base Vec::<line_segment_sketch::LineSegmentStroke>::__call__ as fn(Vec<line_segment_sketch::LineSegmentStroke<'static>>) -> Vec<line_segment_sketch::LineSegmentStroke<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]mnist_classifier::line_segment_sketch::LineSegmentStroke", "i32"],
        },
        index_linkage!(
            mutable,
            Intrinsic,
            BoxNonCopyable,Vec<line_segment_sketch::LineSegmentStroke<'eval>>,
            __registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE,
            line_segment_sketch::LineSegmentStroke<'eval>,
            __registration__::__LINE_SEGMENT_STROKE_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::LineSegmentSketch::new",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                    let r: f32 = __arguments[1].downcast_f32();
                    __Register::new_box::<line_segment_sketch::LineSegmentSketch<'eval>>(line_segment_sketch::LineSegmentSketch::new(ct, r), &__registration__::__LINE_SEGMENT_SKETCH_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::LineSegmentSketch::new as fn(&'static raw_contour::RawContour<'static>, f32) -> line_segment_sketch::LineSegmentSketch<'static>
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::go_right",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let u: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    let r: f32 = __arguments[1].downcast_f32();
                    __Register::new_box::<geom2d::Vector2d>(line_segment_sketch::go_right(u, r), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::go_right as fn(&'static geom2d::Vector2d, f32) -> geom2d::Vector2d
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::go_left",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let u: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    let r: f32 = __arguments[1].downcast_f32();
                    __Register::new_box::<geom2d::Vector2d>(line_segment_sketch::go_left(u, r), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::go_left as fn(&'static geom2d::Vector2d, f32) -> geom2d::Vector2d
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::extend_end",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                    let start: i32 = __arguments[1].downcast_i32();
                    let r: f32 = __arguments[2].downcast_f32();
                    line_segment_sketch::extend_end(ct, start, r).to_register()
                }
                __wrapper
            },
            some base line_segment_sketch::extend_end as fn(&'static raw_contour::RawContour<'static>, i32, f32) -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::norm" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    __this.norm().to_register()
                }
                __wrapper
            },
            some base geom2d::Vector2d::norm as fn(&'static geom2d::Vector2d) -> f32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::extend_start",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                    let start0: i32 = __arguments[1].downcast_i32();
                    let end: i32 = __arguments[2].downcast_i32();
                    let r: f32 = __arguments[3].downcast_f32();
                    line_segment_sketch::extend_start(ct, start0, end, r).to_register()
                }
                __wrapper
            },
            some base line_segment_sketch::extend_start as fn(&'static raw_contour::RawContour<'static>, i32, i32, f32) -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::line_segment_sketch::find_line_segments",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
                    let r: f32 = __arguments[1].downcast_f32();
                    __Register::new_box::<Vec<line_segment_sketch::LineSegmentStroke<'eval>>>(line_segment_sketch::find_line_segments(ct, r), &__registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::find_line_segments as fn(&'static raw_contour::RawContour<'static>, f32) -> Vec<line_segment_sketch::LineSegmentStroke<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>::lastx" },
        method_elem_linkage!(Vec<line_segment_sketch::LineSegmentStroke<'eval>>, __registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE, __registration__::__LINE_SEGMENT_STROKE_VTABLE, lastx)
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::dot" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    __this.dot(other).to_register()
                }
                __wrapper
            },
            some base geom2d::Vector2d::dot as fn(&'static geom2d::Vector2d, &'static geom2d::Vector2d) -> f32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Point2d::to" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::Point2d = __arguments[0].downcast_temp_ref(&__registration__::__POINT_2_D_VTABLE);
                    let other: &geom2d::Point2d = __arguments[1].downcast_temp_ref(&__registration__::__POINT_2_D_VTABLE);
                    __Register::new_box::<geom2d::Vector2d>(__this.to(other), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            some base geom2d::Point2d::to as fn(&'static geom2d::Point2d, &'static geom2d::Point2d) -> geom2d::Vector2d
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>::popx" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<line_segment_sketch::LineSegmentStroke<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE);
                    __Register::new_box::<line_segment_sketch::LineSegmentStroke<'eval>>(__this.popx(), &__registration__::__LINE_SEGMENT_STROKE_VTABLE)
                }
                __wrapper
            },
            some base Vec::<line_segment_sketch::LineSegmentStroke>::popx as fn(&'static mut Vec<line_segment_sketch::LineSegmentStroke<'static>>) -> line_segment_sketch::LineSegmentStroke<'static>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>::push" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<line_segment_sketch::LineSegmentStroke<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE);
                    let element: line_segment_sketch::LineSegmentStroke<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__LINE_SEGMENT_STROKE_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            some base Vec::<line_segment_sketch::LineSegmentStroke>::push as fn(&'static mut Vec<line_segment_sketch::LineSegmentStroke<'static>>, line_segment_sketch::LineSegmentStroke<'static>) -> ()
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>::firstx" },
        method_elem_linkage!(Vec<line_segment_sketch::LineSegmentStroke<'eval>>, __registration__::__VEC_LINE_SEGMENT_STROKE_VTABLE, __registration__::__LINE_SEGMENT_STROKE_VTABLE, firstx)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::fermi::FermiMatchResult"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let matches: Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_move(&__registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE);
                    let others: Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE);
                    __Register::new_box::<fermi::FermiMatchResult<'eval>>(fermi::FermiMatchResult::__call__(matches, others), &__registration__::__FERMI_MATCH_RESULT_VTABLE)
                }
                __wrapper
            },
            some base fermi::FermiMatchResult::__call__ as fn(Vec<Option<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>>, Vec<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>) -> fermi::FermiMatchResult<'static>
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::fermi::FermiMatchResult",
            field_ident: "matches",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE, Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>>, __registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE,
            matches
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::fermi::FermiMatchResult",
            field_ident: "others",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE, Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>, __registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE,
            others
        )
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::fermi::FermiMatchResult",
            field_ident: "norm",
        },
        lazy_field_linkage!(fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE, f32, __registration__::__F32_VTABLE, norm)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::fermi::FermiMatchResult",
            field_ident: "rel_norm",
        },
        lazy_field_linkage!(fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE, f32, __registration__::__F32_VTABLE, rel_norm)
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "mnist_classifier::fermi::FermiMatchResult",
            field_ident: "angle_change_norm",
        },
        lazy_field_linkage!(fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE, f32, __registration__::__F32_VTABLE, angle_change_norm)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<?&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::ilen" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<Option<&line_segment_sketch::concave_component::ConcaveComponent>>::ilen as fn(&'static Vec<Option<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]?&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_opt_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE))
                            .collect();
                    __Register::new_box::<Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>>>(Vec::<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent>>::__call__(__variadics), &__registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some base Vec::<Option<&line_segment_sketch::concave_component::ConcaveComponent>>::__call__ as fn(Vec<Option<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>>) -> Vec<Option<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]?&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent", "i32"],
        },
        index_linkage!(
            mutable,
            OptionalEvalRef,
            BoxNonCopyable,Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>>,
            __registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE,
            line_segment_sketch::concave_component::ConcaveComponent<'eval>,
            __registration__::__CONCAVE_COMPONENT_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::ilen" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<&line_segment_sketch::concave_component::ConcaveComponent>::ilen as fn(&'static Vec<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE))
                            .collect();
                    __Register::new_box::<Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>>(Vec::<&'eval line_segment_sketch::concave_component::ConcaveComponent>::__call__(__variadics), &__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some base Vec::<&line_segment_sketch::concave_component::ConcaveComponent>::__call__ as fn(Vec<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>) -> Vec<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent", "i32"],
        },
        index_linkage!(
            mutable,
            EvalRef,
            BoxNonCopyable,Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>,
            __registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE,
            line_segment_sketch::concave_component::ConcaveComponent<'eval>,
            __registration__::__CONCAVE_COMPONENT_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::fermi::fermi_match",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let concave_components: &'eval Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_eval_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
                    let templates: &Vec<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>> = __arguments[1].downcast_temp_ref(&__registration__::__VEC_THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE);
                    __Register::new_box::<fermi::FermiMatchResult<'eval>>(fermi::fermi_match(concave_components, templates, __opt_ctx.unwrap()), &__registration__::__FERMI_MATCH_RESULT_VTABLE)
                }
                __wrapper
            },
            some ctx fermi::fermi_match as fn(&'static Vec<line_segment_sketch::concave_component::ConcaveComponent<'static>>, &'static Vec<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>>, &dyn __EvalContext<'static>) -> fermi::FermiMatchResult<'static>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<ThickFp<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, ?f32>>::ilen" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>>::ilen as fn(&'static Vec<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]ThickFp<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, ?f32>"
        },

        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v| {
                                v.downcast_temp_ref::<__VirtualFunction>(
                                    &__registration__::__THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE
                                ).downcast_thick_fp()
                            })
                            .collect();
                    __Register::new_box::<Vec<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>>>(Vec::<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>>::__call__(__variadics), &__registration__::__VEC_THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE)
                }
                __wrapper
            },
            some base Vec::<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>>::__call__ as fn(Vec<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>>) -> Vec<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]ThickFp<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, ?f32>", "i32"],
        },
        index_linkage!(
            mutable,
            Intrinsic,
            BoxCopyable,Vec<ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>>,
            __registration__::__VEC_THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE,
            ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>,
            __registration__::__THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE
)
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::collect_refs" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &'eval Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_eval_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
                    __Register::new_box::<Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>>(__this.collect_refs(), &__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some base Vec::<line_segment_sketch::concave_component::ConcaveComponent>::collect_refs as fn(&'static Vec<line_segment_sketch::concave_component::ConcaveComponent<'static>>) -> Vec<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::pop_with_largest_opt_f32" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE);
                    let f: ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>> = unsafe { __arguments[1]
                        .downcast_temp_ref::<__VirtualFunction>(&__registration__::__VIRTUAL_FUNCTION_VTABLE)
                        .downcast_thick_fp() };
                    __Register::new_opt_eval_ref::<line_segment_sketch::concave_component::ConcaveComponent<'eval>>(__this.pop_with_largest_opt_f32_copyable(f, __opt_ctx.unwrap()), &__registration__::__CONCAVE_COMPONENT_VTABLE)
                }
                __wrapper
            },
            some ctx Vec::<&line_segment_sketch::concave_component::ConcaveComponent>::pop_with_largest_opt_f32_copyable as fn(&'static mut Vec<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>, ThickFp<fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>>, &dyn __EvalContext<'static>) -> Option<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<?&mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::push" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE);
                    let element: Option<&'eval line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[1].downcast_opt_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            some base Vec::<Option<&line_segment_sketch::concave_component::ConcaveComponent>>::push as fn(&'static mut Vec<Option<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>>, Option<&'static line_segment_sketch::concave_component::ConcaveComponent<'static>>) -> ()
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::major::connected_components"
        },
        feature_linkage!(major::connected_components, Vec<connected_component::ConnectedComponent>, __registration__::__VEC_CONNECTED_COMPONENT_VTABLE),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::major::major_connected_component"
        },
        feature_linkage!(major::major_connected_component, connected_component::ConnectedComponent, __registration__::__CONNECTED_COMPONENT_VTABLE),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::major::ignored_connected_components_row_span_sum_sum"
        },
        feature_linkage!(major::ignored_connected_components_row_span_sum_sum, f32, __registration__::__F32_VTABLE),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::major::major_raw_contours"
        },
        feature_linkage!(major::major_raw_contours, Vec<raw_contour::RawContour<'eval>>, __registration__::__VEC_RAW_CONTOUR_VTABLE),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::major::major_raw_contour"
        },
        feature_linkage!(major::major_raw_contour, raw_contour::RawContour<'eval>, __registration__::__RAW_CONTOUR_VTABLE),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::major::major_line_segment_sketch"
        },
        feature_linkage!(major::major_line_segment_sketch, line_segment_sketch::LineSegmentSketch<'eval>, __registration__::__LINE_SEGMENT_SKETCH_VTABLE),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::major::major_concave_components"
        },
        feature_linkage!(major::major_concave_components, Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>>, __registration__::__VEC_CONCAVE_COMPONENT_VTABLE),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::zero::open_one_match"
        },
        feature_linkage!(zero::open_one_match, fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::zero::almost_closed",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    zero::almost_closed(cc, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx zero::almost_closed as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::displacement" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    __Register::new_box::<geom2d::Vector2d>(__this.displacement(), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::concave_component::ConcaveComponent::displacement as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> geom2d::Vector2d
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::BoundingBox::ymax" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::BoundingBox = __arguments[0].downcast_temp_ref(&__registration__::__BOUNDING_BOX_VTABLE);
                    __this.ymax().to_register()
                }
                __wrapper
            },
            some base geom2d::BoundingBox::ymax as fn(&'static geom2d::BoundingBox) -> f32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::BoundingBox::ymin" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::BoundingBox = __arguments[0].downcast_temp_ref(&__registration__::__BOUNDING_BOX_VTABLE);
                    __this.ymin().to_register()
                }
                __wrapper
            },
            some base geom2d::BoundingBox::ymin as fn(&'static geom2d::BoundingBox) -> f32
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::one::one_fermi_match"
        },
        feature_linkage!(one::one_fermi_match, fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::one::downmost",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    one::downmost(cc).to_register()
                }
                __wrapper
            },
            some base one::downmost as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::one::upmost",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    one::upmost(cc).to_register()
                }
                __wrapper
            },
            some base one::upmost as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::one::hat",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    one::hat(cc).to_register()
                }
                __wrapper
            },
            some base one::hat as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::connected_component::ConnectedComponent::top_k_row_span_sum" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &connected_component::ConnectedComponent = __arguments[0].downcast_temp_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
                    let k: i32 = __arguments[1].downcast_i32();
                    __this.top_k_row_span_sum(k).to_register()
                }
                __wrapper
            },
            some base connected_component::ConnectedComponent::top_k_row_span_sum as fn(&'static connected_component::ConnectedComponent, i32) -> f32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::end" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    __Register::new_box::<geom2d::Point2d>(__this.end(), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::concave_component::ConcaveComponent::end as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> geom2d::Point2d
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::three::three_fermi_match"
        },
        feature_linkage!(three::three_fermi_match, fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::three::downarc",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    three::downarc(cc, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx three::downarc as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::three::uparc",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    three::uparc(cc, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx three::uparc as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::three::back",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    three::back(cc, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx three::back as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::end_tangent" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    __Register::new_box::<geom2d::Vector2d>(__this.end_tangent(), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::concave_component::ConcaveComponent::end_tangent as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> geom2d::Vector2d
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::angle" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    let is_branch_cut_positive: bool = __arguments[1].downcast_bool();
                    __this.angle(is_branch_cut_positive).to_register()
                }
                __wrapper
            },
            some base geom2d::Vector2d::angle as fn(&'static geom2d::Vector2d, bool) -> f32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::start" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    __Register::new_box::<geom2d::Point2d>(__this.start(), &__registration__::__POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::concave_component::ConcaveComponent::start as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> geom2d::Point2d
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Point2d::dist" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::Point2d = __arguments[0].downcast_temp_ref(&__registration__::__POINT_2_D_VTABLE);
                    let other: &geom2d::Point2d = __arguments[1].downcast_temp_ref(&__registration__::__POINT_2_D_VTABLE);
                    __this.dist(other).to_register()
                }
                __wrapper
            },
            some base geom2d::Point2d::dist as fn(&'static geom2d::Point2d, &'static geom2d::Point2d) -> f32
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::five::five_match"
        },
        feature_linkage!(five::five_match, fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::five::up_right_cc",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    five::up_right_cc(cc, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx five::up_right_cc as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::five::down_left_cc",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    five::down_left_cc(cc, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx five::down_left_cc as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::RelativeBoundingBox::ymin" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::RelativeBoundingBox = __arguments[0].downcast_temp_ref(&__registration__::__RELATIVE_BOUNDING_BOX_VTABLE);
                    __this.ymin().to_register()
                }
                __wrapper
            },
            some base geom2d::RelativeBoundingBox::ymin as fn(&'static geom2d::RelativeBoundingBox) -> f32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::RelativeBoundingBox::ymax" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::RelativeBoundingBox = __arguments[0].downcast_temp_ref(&__registration__::__RELATIVE_BOUNDING_BOX_VTABLE);
                    __this.ymax().to_register()
                }
                __wrapper
            },
            some base geom2d::RelativeBoundingBox::ymax as fn(&'static geom2d::RelativeBoundingBox) -> f32
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::six::six_match"
        },
        feature_linkage!(six::six_match, fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::six::upmost",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    six::upmost(cc).to_register()
                }
                __wrapper
            },
            some base six::upmost as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::six::six_match_refined1"
        },
        feature_linkage!(six::six_match_refined1, fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::six::bottom1",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    six::bottom1(cc, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx six::bottom1 as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::BoundingBox::relative_point" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::BoundingBox = __arguments[0].downcast_temp_ref(&__registration__::__BOUNDING_BOX_VTABLE);
                    let point: &geom2d::Point2d = __arguments[1].downcast_temp_ref(&__registration__::__POINT_2_D_VTABLE);
                    __Register::new_box::<geom2d::RelativePoint2d>(__this.relative_point(point), &__registration__::__RELATIVE_POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base geom2d::BoundingBox::relative_point as fn(&'static geom2d::BoundingBox, &'static geom2d::Point2d) -> geom2d::RelativePoint2d
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::eight::upper_mouth_match"
        },
        feature_linkage!(eight::upper_mouth_match, fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::eight::big_mouth",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    eight::big_mouth(cc, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx eight::big_mouth as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegmentStroke>::firstx" },
        method_elem_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>>, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE, __registration__::__LINE_SEGMENT_STROKE_VTABLE, firstx)
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::nine::nine_match"
        },
        feature_linkage!(nine::nine_match, fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::nine::downmost",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    nine::downmost(cc).to_register()
                }
                __wrapper
            },
            some base nine::downmost as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "mnist_classifier::nine::nine_match_refine"
        },
        feature_linkage!(nine::nine_match_refine, fermi::FermiMatchResult<'eval>, __registration__::__FERMI_MATCH_RESULT_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "mnist_classifier::nine::big_cc",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let cc: &'eval line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_eval_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    nine::big_cc(cc, __opt_ctx.unwrap()).to_register()
                }
                __wrapper
            },
            some ctx nine::big_cc as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::connected_component::ConnectedComponent::top_k_row_right_mass_sum" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &connected_component::ConnectedComponent = __arguments[0].downcast_temp_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
                    let k: i32 = __arguments[1].downcast_i32();
                    __this.top_k_row_right_mass_sum(k).to_register()
                }
                __wrapper
            },
            some base connected_component::ConnectedComponent::top_k_row_right_mass_sum as fn(&'static connected_component::ConnectedComponent, i32) -> f32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "Vec<mnist_classifier::geom2d::Point2d>::cyclic_slice" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &'eval Vec<geom2d::Point2d> = __arguments[0].downcast_eval_ref(&__registration__::__VEC_POINT_2_D_VTABLE);
                    let start: i32 = __arguments[1].downcast_i32();
                    let end: i32 = __arguments[2].downcast_i32();
                    __Register::new_box::<__std::slice::CyclicSlice<'eval, geom2d::Point2d>>(__this.cyclic_slice(start, end), &__registration__::__CYCLIC_SLICE_POINT_2_D_VTABLE)
                }
                __wrapper
            },
            some base Vec::<geom2d::Point2d>::cyclic_slice as fn(&'static Vec<geom2d::Point2d>, i32, i32) -> __std::slice::CyclicSlice<'static, geom2d::Point2d>
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::Vector2d::angle_to" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref(&__registration__::__VECTOR_2_D_VTABLE);
                    let is_branch_cut_positive: bool = __arguments[2].downcast_bool();
                    __this.angle_to(other, is_branch_cut_positive).to_register()
                }
                __wrapper
            },
            some base geom2d::Vector2d::angle_to as fn(&'static geom2d::Vector2d, &'static geom2d::Vector2d, bool) -> f32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::line_segment" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &line_segment_sketch::concave_component::ConcaveComponent<'eval> = __arguments[0].downcast_temp_ref(&__registration__::__CONCAVE_COMPONENT_VTABLE);
                    __Register::new_box::<line_segment_sketch::line_segment::LineSegment>(__this.line_segment(), &__registration__::__LINE_SEGMENT_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::concave_component::ConcaveComponent::line_segment as fn(&'static line_segment_sketch::concave_component::ConcaveComponent<'static>) -> line_segment_sketch::line_segment::LineSegment
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::line_segment_sketch::line_segment::LineSegment::displacement" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &line_segment_sketch::line_segment::LineSegment = __arguments[0].downcast_temp_ref(&__registration__::__LINE_SEGMENT_VTABLE);
                    __Register::new_box::<geom2d::Vector2d>(__this.displacement(), &__registration__::__VECTOR_2_D_VTABLE)
                }
                __wrapper
            },
            some base line_segment_sketch::line_segment::LineSegment::displacement as fn(&'static line_segment_sketch::line_segment::LineSegment) -> geom2d::Vector2d
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegmentStroke>::lastx" },
        method_elem_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegmentStroke<'eval>>, __registration__::__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE, __registration__::__LINE_SEGMENT_STROKE_VTABLE, lastx)
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::BoundingBox::relative_bounding_box" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::BoundingBox = __arguments[0].downcast_temp_ref(&__registration__::__BOUNDING_BOX_VTABLE);
                    let other: &geom2d::BoundingBox = __arguments[1].downcast_temp_ref(&__registration__::__BOUNDING_BOX_VTABLE);
                    __Register::new_box::<geom2d::RelativeBoundingBox>(__this.relative_bounding_box(other), &__registration__::__RELATIVE_BOUNDING_BOX_VTABLE)
                }
                __wrapper
            },
            some base geom2d::BoundingBox::relative_bounding_box as fn(&'static geom2d::BoundingBox, &'static geom2d::BoundingBox) -> geom2d::RelativeBoundingBox
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::ClosedRange::relative_point" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::ClosedRange = __arguments[0].downcast_temp_ref(&__registration__::__CLOSED_RANGE_VTABLE);
                    let v: f32 = __arguments[1].downcast_f32();
                    __this.relative_point(v).to_register()
                }
                __wrapper
            },
            some base geom2d::ClosedRange::relative_point as fn(&'static geom2d::ClosedRange, f32) -> f32
        ),
    ),
    (
        __StaticLinkageKey::Routine { route: "mnist_classifier::geom2d::ClosedRange::relative_range" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &geom2d::ClosedRange = __arguments[0].downcast_temp_ref(&__registration__::__CLOSED_RANGE_VTABLE);
                    let other: &geom2d::ClosedRange = __arguments[1].downcast_temp_ref(&__registration__::__CLOSED_RANGE_VTABLE);
                    __Register::new_box::<geom2d::ClosedRange>(__this.relative_range(other), &__registration__::__CLOSED_RANGE_VTABLE)
                }
                __wrapper
            },
            some base geom2d::ClosedRange::relative_range as fn(&'static geom2d::ClosedRange, &'static geom2d::ClosedRange) -> geom2d::ClosedRange
        ),
    ),
];
