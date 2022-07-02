
use crate::*;

pub fn link_entity_with_compiled(compile_time: &mut husky_compile_time::HuskyCompileTime) {
    compile_time.load_linkages(&[
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::connected_component::ConnectedComponent"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "mask",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::connected_component::horizontal_extend"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::connected_component::find_connected_components"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::raw_contour::RawContour"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "cc",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::raw_contour::RawContour",
            field_ident: "points",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_pixel_pair"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_pixel_to_the_left"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_pixel_to_the_right"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_inward_direction"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_angle_change"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_outward_direction"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::raw_contour::StreakCache"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::raw_contour::StreakCache",
            field_ident: "prev1",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::raw_contour::StreakCache",
            field_ident: "prev2",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::get_concave_middle_point"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::raw_contour::find_raw_contours"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::Point2d"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::geom2d::Point2d",
            field_ident: "x",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::geom2d::Point2d",
            field_ident: "y",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::geom2d::Vector2d"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::geom2d::Vector2d",
            field_ident: "x",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::geom2d::Vector2d",
            field_ident: "y",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::LineSegment"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegment",
            field_ident: "points",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegment",
            field_ident: "start",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegment",
            field_ident: "end",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "contour",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch",
            field_ident: "line_segments",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::go_right"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::go_left"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::extend_end"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::extend_start"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::find_line_segments"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::geom2d::Point2d::from_i_shift28"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "line_segment_sketch",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent",
            field_ident: "line_segments",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::concave_component::find_concave_components"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent",
            field_ident: "line_segment_sketch",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent",
            field_ident: "line_segments",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::convexity::is_convex"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "start",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "end",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::LineSegment::new"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>",
            field_ident: "start",
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>",
            field_ident: "end",
        },
        todo!(),
    ),
    ])
}
