Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `mnist_classifier`,
                module_symbols: [
                    Module {
                        ident: `connected_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier`,
                        ),
                        ast_idx: 12,
                        module_path: `mnist_classifier::connected_component`,
                    },
                    Module {
                        ident: `raw_contour`,
                        accessibility: PubicUnder(
                            `mnist_classifier`,
                        ),
                        ast_idx: 13,
                        module_path: `mnist_classifier::raw_contour`,
                    },
                    Module {
                        ident: `geom2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier`,
                        ),
                        ast_idx: 14,
                        module_path: `mnist_classifier::geom2d`,
                    },
                    Module {
                        ident: `line_segment_sketch`,
                        accessibility: PubicUnder(
                            `mnist_classifier`,
                        ),
                        ast_idx: 15,
                        module_path: `mnist_classifier::line_segment_sketch`,
                    },
                    Module {
                        ident: `fermi`,
                        accessibility: PubicUnder(
                            `mnist_classifier`,
                        ),
                        ast_idx: 16,
                        module_path: `mnist_classifier::fermi`,
                    },
                    Module {
                        ident: `major`,
                        accessibility: PubicUnder(
                            `mnist_classifier`,
                        ),
                        ast_idx: 17,
                        module_path: `mnist_classifier::major`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::connected_component`,
                module_symbols: [
                    ModuleItem {
                        ident: `ConnectedComponentDistribution`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 121,
                        path: `mnist_classifier::connected_component::ConnectedComponentDistribution`,
                    },
                    ModuleItem {
                        ident: `EffHoles`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 122,
                        path: `mnist_classifier::connected_component::EffHoles`,
                    },
                    ModuleItem {
                        ident: `hole_tmpl`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 123,
                        path: `mnist_classifier::connected_component::hole_tmpl`,
                    },
                    ModuleItem {
                        ident: `ConnectedComponent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 124,
                        path: `mnist_classifier::connected_component::ConnectedComponent`,
                    },
                    ModuleItem {
                        ident: `horizontal_extend`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 125,
                        path: `mnist_classifier::connected_component::horizontal_extend`,
                    },
                    ModuleItem {
                        ident: `find_connected_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 126,
                        path: `mnist_classifier::connected_component::find_connected_components`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::raw_contour`,
                module_symbols: [
                    ModuleItem {
                        ident: `RawContour`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 217,
                        path: `mnist_classifier::raw_contour::RawContour`,
                    },
                    ModuleItem {
                        ident: `Direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 219,
                        path: `mnist_classifier::raw_contour::Direction`,
                    },
                    ModuleItem {
                        ident: `get_pixel_pair`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 224,
                        path: `mnist_classifier::raw_contour::get_pixel_pair`,
                    },
                    ModuleItem {
                        ident: `get_pixel_to_the_left`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 225,
                        path: `mnist_classifier::raw_contour::get_pixel_to_the_left`,
                    },
                    ModuleItem {
                        ident: `get_pixel_to_the_right`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 226,
                        path: `mnist_classifier::raw_contour::get_pixel_to_the_right`,
                    },
                    ModuleItem {
                        ident: `get_inward_direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 227,
                        path: `mnist_classifier::raw_contour::get_inward_direction`,
                    },
                    ModuleItem {
                        ident: `get_angle_change`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 228,
                        path: `mnist_classifier::raw_contour::get_angle_change`,
                    },
                    ModuleItem {
                        ident: `get_outward_direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 229,
                        path: `mnist_classifier::raw_contour::get_outward_direction`,
                    },
                    ModuleItem {
                        ident: `StreakCache`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 230,
                        path: `mnist_classifier::raw_contour::StreakCache`,
                    },
                    ModuleItem {
                        ident: `get_concave_middle_point`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 231,
                        path: `mnist_classifier::raw_contour::get_concave_middle_point`,
                    },
                    ModuleItem {
                        ident: `find_raw_contours`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 232,
                        path: `mnist_classifier::raw_contour::find_raw_contours`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::geom2d`,
                module_symbols: [
                    ModuleItem {
                        ident: `Point2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 79,
                        path: `mnist_classifier::geom2d::Point2d`,
                    },
                    ModuleItem {
                        ident: `RelativePoint2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 81,
                        path: `mnist_classifier::geom2d::RelativePoint2d`,
                    },
                    ModuleItem {
                        ident: `Vector2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 82,
                        path: `mnist_classifier::geom2d::Vector2d`,
                    },
                    ModuleItem {
                        ident: `ClosedRange`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 84,
                        path: `mnist_classifier::geom2d::ClosedRange`,
                    },
                    ModuleItem {
                        ident: `BoundingBox`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 86,
                        path: `mnist_classifier::geom2d::BoundingBox`,
                    },
                    ModuleItem {
                        ident: `RelativeBoundingBox`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 88,
                        path: `mnist_classifier::geom2d::RelativeBoundingBox`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch`,
                module_symbols: [
                    Module {
                        ident: `concave_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 160,
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                    },
                    Module {
                        ident: `convex_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 161,
                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                    },
                    Module {
                        ident: `convexity`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 162,
                        module_path: `mnist_classifier::line_segment_sketch::convexity`,
                    },
                    Module {
                        ident: `line_segment`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 163,
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                    },
                    ModuleItem {
                        ident: `LineSegmentStroke`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 169,
                        path: `mnist_classifier::line_segment_sketch::LineSegmentStroke`,
                    },
                    ModuleItem {
                        ident: `LineSegmentSketch`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 171,
                        path: `mnist_classifier::line_segment_sketch::LineSegmentSketch`,
                    },
                    ModuleItem {
                        ident: `go_right`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 173,
                        path: `mnist_classifier::line_segment_sketch::go_right`,
                    },
                    ModuleItem {
                        ident: `go_left`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 174,
                        path: `mnist_classifier::line_segment_sketch::go_left`,
                    },
                    ModuleItem {
                        ident: `extend_end`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 175,
                        path: `mnist_classifier::line_segment_sketch::extend_end`,
                    },
                    ModuleItem {
                        ident: `extend_start`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 176,
                        path: `mnist_classifier::line_segment_sketch::extend_start`,
                    },
                    ModuleItem {
                        ident: `find_line_segments`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 177,
                        path: `mnist_classifier::line_segment_sketch::find_line_segments`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                module_symbols: [
                    ModuleItem {
                        ident: `ConcaveComponent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 77,
                        path: `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`,
                    },
                    ModuleItem {
                        ident: `find_concave_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 78,
                        path: `mnist_classifier::line_segment_sketch::concave_component::find_concave_components`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                module_symbols: [
                    ModuleItem {
                        ident: `ConvexCompoent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::convex_component`,
                        ),
                        ast_idx: 5,
                        path: `mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::convexity`,
                module_symbols: [
                    ModuleItem {
                        ident: `is_convex`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::convexity`,
                        ),
                        ast_idx: 23,
                        path: `mnist_classifier::line_segment_sketch::convexity::is_convex`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                module_symbols: [
                    ModuleItem {
                        ident: `LineSegment`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 16,
                        path: `mnist_classifier::line_segment_sketch::line_segment::LineSegment`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::fermi`,
                module_symbols: [
                    ModuleItem {
                        ident: `FermiMatchResult`,
                        accessibility: PubicUnder(
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 24,
                        path: `mnist_classifier::fermi::FermiMatchResult`,
                    },
                    ModuleItem {
                        ident: `fermi_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 26,
                        path: `mnist_classifier::fermi::fermi_match`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::major`,
                module_symbols: [
                    ModuleItem {
                        ident: `connected_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 22,
                        path: `mnist_classifier::major::connected_components`,
                    },
                    ModuleItem {
                        ident: `major_connected_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 23,
                        path: `mnist_classifier::major::major_connected_component`,
                    },
                    ModuleItem {
                        ident: `ignored_connected_components_row_span_sum_sum`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 24,
                        path: `mnist_classifier::major::ignored_connected_components_row_span_sum_sum`,
                    },
                    ModuleItem {
                        ident: `major_raw_contours`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 25,
                        path: `mnist_classifier::major::major_raw_contours`,
                    },
                    ModuleItem {
                        ident: `major_raw_contour`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 27,
                        path: `mnist_classifier::major::major_raw_contour`,
                    },
                    ModuleItem {
                        ident: `major_line_segment_sketch`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 28,
                        path: `mnist_classifier::major::major_line_segment_sketch`,
                    },
                    ModuleItem {
                        ident: `major_concave_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 29,
                        path: `mnist_classifier::major::major_concave_components`,
                    },
                ],
            },
        ],
    },
)