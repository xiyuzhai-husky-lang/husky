
use crate::*;
use __husky_root::__init_utils::*;

pub fn link_entity_with_compiled(compile_time: &mut husky_compile_time::HuskyCompileTime) {
    compile_time.load_linkages(&[
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::connected_component::ConnectedComponent"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let mask: domains::ml::datasets::cv::mnist::BinaryImage28 = unsafe { __arb_ref(&__arguments[0]) }.downcast_move();
                __TempValue::OwnedEval(__OwnedValue::new(
                    connected_component::ConnectedComponent::__call__(mask)
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "mask",
        },
        field_linkage!(connected_component::ConnectedComponent, mask)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::raw_contour::RawContour"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let cc: &'eval connected_component::ConnectedComponent = __arguments[0].downcast_eval_ref();
                let points: Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::OwnedEval(__OwnedValue::new(
                    raw_contour::RawContour::__call__(cc, points)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "cc",
        },
        field_linkage!(raw_contour::RawContour<'eval>, cc)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "points",
        },
        field_linkage!(raw_contour::RawContour<'eval>, points)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "Vec<mnist_classifier::raw_contour::RawContour>"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::OwnedEval(__OwnedValue::new(
                    Vec::<raw_contour::RawContour>::__call__()
                    ))
            }
            __wrapper
        }, 0),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::raw_contour::RawContour>::ilen"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &Vec<raw_contour::RawContour<'eval>> = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.ilen()
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::raw_contour::RawContour>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<raw_contour::RawContour<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: raw_contour::RawContour<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::raw_contour::RawContour>::popx"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<raw_contour::RawContour<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.popx()
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::raw_contour::RawContour>::firstx"
        },
        method_elem_linkage!(Vec<raw_contour::RawContour<'eval>>, firstx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::raw_contour::RawContour>::lastx"
        },
        method_elem_linkage!(Vec<raw_contour::RawContour<'eval>>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::raw_contour::RawContour>::cyclic_slice"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &'eval Vec<raw_contour::RawContour<'eval>> = __arguments[0].downcast_eval_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.cyclic_slice(start, end)
                    ))
            }
            __wrapper
        }, 3),

    ),
    (

        __StaticLinkageKey::Index {
            opd_tys: &["Vec<mnist_classifier::raw_contour::RawContour>", "i32"],
        },
        index_linkage!(Vec<raw_contour::RawContour<'eval>>)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "Vec<mnist_classifier::connected_component::ConnectedComponent>"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::OwnedEval(__OwnedValue::new(
                    Vec::<connected_component::ConnectedComponent>::__call__()
                    ))
            }
            __wrapper
        }, 0),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::connected_component::ConnectedComponent>::ilen"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &Vec<connected_component::ConnectedComponent> = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.ilen()
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::connected_component::ConnectedComponent>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<connected_component::ConnectedComponent> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: connected_component::ConnectedComponent = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::connected_component::ConnectedComponent>::popx"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<connected_component::ConnectedComponent> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.popx()
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::connected_component::ConnectedComponent>::firstx"
        },
        method_elem_linkage!(Vec<connected_component::ConnectedComponent>, firstx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::connected_component::ConnectedComponent>::lastx"
        },
        method_elem_linkage!(Vec<connected_component::ConnectedComponent>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::connected_component::ConnectedComponent>::cyclic_slice"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &'eval Vec<connected_component::ConnectedComponent> = __arguments[0].downcast_eval_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.cyclic_slice(start, end)
                    ))
            }
            __wrapper
        }, 3),

    ),
    (

        __StaticLinkageKey::Index {
            opd_tys: &["Vec<mnist_classifier::connected_component::ConnectedComponent>", "i32"],
        },
        index_linkage!(Vec<connected_component::ConnectedComponent>)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::connected_component::horizontal_extend"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let a: u32 = __arguments[0].downcast_copy();
                let x: u32 = __arguments[1].downcast_copy();
                __TempValue::Copyable(
                    connected_component::horizontal_extend(a, x)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::connected_component::find_connected_components"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let img: &domains::ml::datasets::cv::mnist::BinaryImage28 = __arguments[0].downcast_temp_ref();
                __TempValue::OwnedEval(__OwnedValue::new(
                    connected_component::find_connected_components(img)
                    ))
            }
            __wrapper
        }, 1),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::connected_component::ConnectedComponent>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<connected_component::ConnectedComponent> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: connected_component::ConnectedComponent = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::Point2d"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let x: f32 = __arguments[0].downcast_copy();
                let y: f32 = __arguments[1].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    geom2d::Point2d::__call__(x, y)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::geom2d::Point2d",
            field_ident: "x",
        },
        field_linkage!(geom2d::Point2d, x)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::geom2d::Point2d",
            field_ident: "y",
        },
        field_linkage!(geom2d::Point2d, y)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "Vec<mnist_classifier::geom2d::Point2d>"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::OwnedEval(__OwnedValue::new(
                    Vec::<geom2d::Point2d>::__call__()
                    ))
            }
            __wrapper
        }, 0),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::ilen"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &Vec<geom2d::Point2d> = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.ilen()
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: geom2d::Point2d = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::popx"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.popx()
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::firstx"
        },
        method_elem_linkage!(Vec<geom2d::Point2d>, firstx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::lastx"
        },
        method_elem_linkage!(Vec<geom2d::Point2d>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::cyclic_slice"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &'eval Vec<geom2d::Point2d> = __arguments[0].downcast_eval_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.cyclic_slice(start, end)
                    ))
            }
            __wrapper
        }, 3),

    ),
    (

        __StaticLinkageKey::Index {
            opd_tys: &["Vec<mnist_classifier::geom2d::Point2d>", "i32"],
        },
        index_linkage!(Vec<geom2d::Point2d>)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let contour: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref();
                let line_segments: Vec<line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::LineSegmentSketch::__call__(contour, line_segments)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "contour",
        },
        field_linkage!(line_segment_sketch::LineSegmentSketch<'eval>, contour)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "line_segments",
        },
        field_linkage!(line_segment_sketch::LineSegmentSketch<'eval>, line_segments)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_pixel_pair"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let row: u32 = __arguments[0].downcast_copy();
                let j: i32 = __arguments[1].downcast_copy();
                __TempValue::Copyable(
                    raw_contour::get_pixel_pair(row, j)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_pixel_to_the_left"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let row: u32 = __arguments[0].downcast_copy();
                let j: i32 = __arguments[1].downcast_copy();
                __TempValue::Copyable(
                    raw_contour::get_pixel_to_the_left(row, j)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_pixel_to_the_right"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let row: u32 = __arguments[0].downcast_copy();
                let j: i32 = __arguments[1].downcast_copy();
                __TempValue::Copyable(
                    raw_contour::get_pixel_to_the_right(row, j)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_inward_direction"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let row_above: u32 = __arguments[0].downcast_copy();
                let row_below: u32 = __arguments[1].downcast_copy();
                let j: i32 = __arguments[2].downcast_copy();
                __TempValue::Copyable(
                    raw_contour::get_inward_direction(row_above, row_below, j)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 3),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_angle_change"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let inward: raw_contour::Direction = __arguments[0].downcast_copy();
                let outward: raw_contour::Direction = __arguments[1].downcast_copy();
                __TempValue::Copyable(
                    raw_contour::get_angle_change(inward, outward)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_outward_direction"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let row_above: u32 = __arguments[0].downcast_copy();
                let row_below: u32 = __arguments[1].downcast_copy();
                let j: i32 = __arguments[2].downcast_copy();
                let inward_direction: raw_contour::Direction = __arguments[3].downcast_copy();
                __TempValue::Copyable(
                    raw_contour::get_outward_direction(row_above, row_below, j, inward_direction)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 4),

    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::raw_contour::StreakCache"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let prev1: i32 = __arguments[0].downcast_copy();
                let prev2: i32 = __arguments[1].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    raw_contour::StreakCache::__call__(prev1, prev2)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::raw_contour::StreakCache",
            field_ident: "prev1",
        },
        field_linkage!(raw_contour::StreakCache, prev1)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::raw_contour::StreakCache",
            field_ident: "prev2",
        },
        field_linkage!(raw_contour::StreakCache, prev2)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_concave_middle_point"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let points: &Vec<geom2d::Point2d> = __arguments[0].downcast_temp_ref();
                __TempValue::OwnedEval(__OwnedValue::new(
                    raw_contour::get_concave_middle_point(points)
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::ilen"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &Vec<geom2d::Point2d> = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.ilen()
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::find_raw_contours"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let cc: &'eval connected_component::ConnectedComponent = __arguments[0].downcast_eval_ref();
                __TempValue::OwnedEval(__OwnedValue::new(
                    raw_contour::find_raw_contours(cc)
                    ))
            }
            __wrapper
        }, 1),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::lastx"
        },
        method_elem_linkage!(Vec<geom2d::Point2d>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::geom2d::Point2d::from_i_shift28"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let i: i32 = __arguments[0].downcast_copy();
                let shift: i32 = __arguments[1].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    geom2d::Point2d::from_i_shift28(i, shift)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: geom2d::Point2d = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::popx"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.popx()
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::raw_contour::RawContour>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<raw_contour::RawContour<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: raw_contour::RawContour<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::Vector2d"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let x: f32 = __arguments[0].downcast_copy();
                let y: f32 = __arguments[1].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    geom2d::Vector2d::__call__(x, y)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::geom2d::Vector2d",
            field_ident: "x",
        },
        field_linkage!(geom2d::Vector2d, x)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::geom2d::Vector2d",
            field_ident: "y",
        },
        field_linkage!(geom2d::Vector2d, y)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let line_segment_sketch: &'eval line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_eval_ref();
                let line_segments: __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::concave_component::ConcaveComponent::__call__(line_segment_sketch, line_segments)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "line_segment_sketch",
        },
        field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, line_segment_sketch)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "line_segments",
        },
        field_linkage!(line_segment_sketch::concave_component::ConcaveComponent<'eval>, line_segments)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::LineSegment"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let points: __std::slice::CyclicSlice<'eval, geom2d::Point2d> = unsafe { __arb_ref(&__arguments[0]) }.downcast_move();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::LineSegment::__call__(points)
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegment",
            field_ident: "points",
        },
        field_linkage!(line_segment_sketch::LineSegment<'eval>, points)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegment",
            field_ident: "start",
        },
        field_linkage!(line_segment_sketch::LineSegment<'eval>, start)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegment",
            field_ident: "end",
        },
        field_linkage!(line_segment_sketch::LineSegment<'eval>, end)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>",
            field_ident: "start",
        },
        mut_field_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>>, start)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>",
            field_ident: "end",
        },
        mut_field_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>>, end)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>::firstx"
        },
        method_elem_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>>, firstx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>::lastx"
        },
        method_elem_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::concave_component::find_concave_components"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let line_segment_sketch: &'eval line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_eval_ref();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::concave_component::find_concave_components(line_segment_sketch)
                    ))
            }
            __wrapper
        }, 1),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::OwnedEval(__OwnedValue::new(
                    Vec::<line_segment_sketch::concave_component::ConcaveComponent>::__call__()
                    ))
            }
            __wrapper
        }, 0),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::ilen"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.ilen()
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: line_segment_sketch::concave_component::ConcaveComponent<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::popx"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.popx()
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::firstx"
        },
        method_elem_linkage!(Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>>, firstx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::lastx"
        },
        method_elem_linkage!(Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::cyclic_slice"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &'eval Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = __arguments[0].downcast_eval_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.cyclic_slice(start, end)
                    ))
            }
            __wrapper
        }, 3),

    ),
    (

        __StaticLinkageKey::Index {
            opd_tys: &["Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>", "i32"],
        },
        index_linkage!(Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>>)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::ilen"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &Vec<line_segment_sketch::LineSegment<'eval>> = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.ilen()
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::convexity::is_convex"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let line_segment_sketch: &line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_temp_ref();
                let index: i32 = __arguments[1].downcast_copy();
                __TempValue::Copyable(
                    line_segment_sketch::convexity::is_convex(line_segment_sketch, index)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::cyclic_slice"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &'eval Vec<line_segment_sketch::LineSegment<'eval>> = __arguments[0].downcast_eval_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.cyclic_slice(start, end)
                    ))
            }
            __wrapper
        }, 3),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<line_segment_sketch::concave_component::ConcaveComponent<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: line_segment_sketch::concave_component::ConcaveComponent<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let line_segment_sketch: &'eval line_segment_sketch::LineSegmentSketch<'eval> = __arguments[0].downcast_eval_ref();
                let line_segments: __std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::convex_component::ConvexCompoent::__call__(line_segment_sketch, line_segments)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent",
            field_ident: "line_segment_sketch",
        },
        field_linkage!(line_segment_sketch::convex_component::ConvexCompoent<'eval>, line_segment_sketch)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent",
            field_ident: "line_segments",
        },
        field_linkage!(line_segment_sketch::convex_component::ConvexCompoent<'eval>, line_segments)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::LineSegment::displacement"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &line_segment_sketch::LineSegment<'eval> = __arguments[0].downcast_temp_ref();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.displacement()
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::geom2d::Vector2d::rotation_direction_to"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref();
                let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.rotation_direction_to(other)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::RawContour::displacement"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &raw_contour::RawContour<'eval> = __arguments[0].downcast_temp_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.displacement(start, end)
                    ))
            }
            __wrapper
        }, 3),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::geom2d::Vector2d::cross"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref();
                let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.cross(other)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "start",
        },
        mut_field_linkage!(__std::slice::CyclicSlice<'eval, geom2d::Point2d>, start)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "end",
        },
        mut_field_linkage!(__std::slice::CyclicSlice<'eval, geom2d::Point2d>, end)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>::firstx"
        },
        method_elem_linkage!(__std::slice::CyclicSlice<'eval, geom2d::Point2d>, firstx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>::lastx"
        },
        method_elem_linkage!(__std::slice::CyclicSlice<'eval, geom2d::Point2d>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::LineSegment::new"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref();
                let from: i32 = __arguments[1].downcast_copy();
                let to: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::LineSegment::new(ct, from, to)
                    ))
            }
            __wrapper
        }, 3),

    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "Vec<mnist_classifier::line_segment_sketch::LineSegment>"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::OwnedEval(__OwnedValue::new(
                    Vec::<line_segment_sketch::LineSegment>::__call__()
                    ))
            }
            __wrapper
        }, 0),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::ilen"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &Vec<line_segment_sketch::LineSegment<'eval>> = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.ilen()
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: line_segment_sketch::LineSegment<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::popx"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.popx()
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::firstx"
        },
        method_elem_linkage!(Vec<line_segment_sketch::LineSegment<'eval>>, firstx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::lastx"
        },
        method_elem_linkage!(Vec<line_segment_sketch::LineSegment<'eval>>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::cyclic_slice"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &'eval Vec<line_segment_sketch::LineSegment<'eval>> = __arguments[0].downcast_eval_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.cyclic_slice(start, end)
                    ))
            }
            __wrapper
        }, 3),

    ),
    (

        __StaticLinkageKey::Index {
            opd_tys: &["Vec<mnist_classifier::line_segment_sketch::LineSegment>", "i32"],
        },
        index_linkage!(Vec<line_segment_sketch::LineSegment<'eval>>)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::LineSegmentSketch::new"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref();
                let r: f32 = __arguments[1].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::LineSegmentSketch::new(ct, r)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::go_right"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let u: &geom2d::Vector2d = __arguments[0].downcast_temp_ref();
                let r: f32 = __arguments[1].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::go_right(u, r)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::go_left"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let u: &geom2d::Vector2d = __arguments[0].downcast_temp_ref();
                let r: f32 = __arguments[1].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::go_left(u, r)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::extend_end"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let r: f32 = __arguments[2].downcast_copy();
                __TempValue::Copyable(
                    line_segment_sketch::extend_end(ct, start, r)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 3),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::geom2d::Vector2d::norm"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.norm()
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::extend_start"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref();
                let start0: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                let r: f32 = __arguments[3].downcast_copy();
                __TempValue::Copyable(
                    line_segment_sketch::extend_start(ct, start0, end, r)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 4),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::find_line_segments"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let ct: &'eval raw_contour::RawContour<'eval> = __arguments[0].downcast_eval_ref();
                let r: f32 = __arguments[1].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    line_segment_sketch::find_line_segments(ct, r)
                    ))
            }
            __wrapper
        }, 2),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::lastx"
        },
        method_elem_linkage!(Vec<line_segment_sketch::LineSegment<'eval>>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::geom2d::Vector2d::dot"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &geom2d::Vector2d = __arguments[0].downcast_temp_ref();
                let other: &geom2d::Vector2d = __arguments[1].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.dot(other)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::geom2d::Point2d::to"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &geom2d::Point2d = __arguments[0].downcast_temp_ref();
                let other: &geom2d::Point2d = __arguments[1].downcast_temp_ref();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.to(other)
                    ))
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::popx"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.popx()
                    ))
            }
            __wrapper
        }, 1),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<line_segment_sketch::LineSegment<'eval>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: line_segment_sketch::LineSegment<'eval> = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }, 2),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::line_segment_sketch::LineSegment>::firstx"
        },
        method_elem_linkage!(Vec<line_segment_sketch::LineSegment<'eval>>, firstx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<mnist_classifier::geom2d::Point2d>::cyclic_slice"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &'eval Vec<geom2d::Point2d> = __arguments[0].downcast_eval_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.cyclic_slice(start, end)
                    ))
            }
            __wrapper
        }, 3),

    ),
    ])
}
