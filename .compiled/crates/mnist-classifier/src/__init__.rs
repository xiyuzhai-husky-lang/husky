use crate::*;
use __husky_root::__init_utils::*;

pub fn link_entity_with_compiled(compile_time: &mut husky_compile_time::HuskyCompileTime) {
    compile_time.load_linkages(&[
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::connected_component::ConnectedComponent"
        },
        __Linkage::SpecificTransfer(todo!()),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "mnist_classifier::connected_component::ConnectedComponent",
            field_ident: "mask",
        },
        field_linkage!(connected_component::ConnectedComponent, mask)
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
        __Linkage::SpecificTransfer(todo!()),
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
        __Linkage::SpecificTransfer(todo!()),
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
        __Linkage::SpecificTransfer(todo!()),
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
            ty: "mnist_classifier::geom2d::Vector2d"
        },
        __Linkage::SpecificTransfer(todo!()),
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
            ty: "mnist_classifier::line_segment_sketch::LineSegment"
        },
        __Linkage::SpecificTransfer(todo!()),
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
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::LineSegmentSketch"
        },
        __Linkage::SpecificTransfer(todo!()),
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
        __Linkage::SpecificTransfer(todo!()),
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
        __StaticLinkageKey::Routine {
            routine: "mnist_classifier::line_segment_sketch::concave_component::find_concave_components"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent"
        },
        __Linkage::SpecificTransfer(todo!()),
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
            routine: "mnist_classifier::line_segment_sketch::convexity::is_convex"
        },
        todo!(),
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "start",
        },
        field_linkage!(__std::slice::CyclicSlice<'eval, geom2d::Point2d>, start)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::geom2d::Point2d>",
            field_ident: "end",
        },
        field_linkage!(__std::slice::CyclicSlice<'eval, geom2d::Point2d>, end)
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
        field_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>>, start)
    ),
    (
        __StaticLinkageKey::StructFieldAccess {
            this_ty: "std::slice::CyclicSlice<mnist_classifier::line_segment_sketch::LineSegment>",
            field_ident: "end",
        },
        field_linkage!(__std::slice::CyclicSlice<'eval, line_segment_sketch::LineSegment<'eval>>, end)
    ),
    ])
}
