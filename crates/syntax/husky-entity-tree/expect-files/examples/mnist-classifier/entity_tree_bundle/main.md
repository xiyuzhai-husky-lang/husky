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
                        ident: `raw_contours`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 73,
                        path: `mnist_classifier::connected_component::raw_contours`,
                    },
                    ModuleItem {
                        ident: `eff_holes`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 74,
                        path: `mnist_classifier::connected_component::eff_holes`,
                    },
                    ModuleItem {
                        ident: `max_hole_ilen`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 75,
                        path: `mnist_classifier::connected_component::max_hole_ilen`,
                    },
                    ModuleItem {
                        ident: `max_row_span`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 76,
                        path: `mnist_classifier::connected_component::max_row_span`,
                    },
                    ModuleItem {
                        ident: `row_span_sum`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 77,
                        path: `mnist_classifier::connected_component::row_span_sum`,
                    },
                    ModuleItem {
                        ident: `distribution`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 78,
                        path: `mnist_classifier::connected_component::distribution`,
                    },
                    ModuleItem {
                        ident: `upper_mass`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 79,
                        path: `mnist_classifier::connected_component::upper_mass`,
                    },
                    ModuleItem {
                        ident: `lower_mass`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 80,
                        path: `mnist_classifier::connected_component::lower_mass`,
                    },
                    ModuleItem {
                        ident: `top_k_row_span_sum`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 81,
                        path: `mnist_classifier::connected_component::top_k_row_span_sum`,
                    },
                    ModuleItem {
                        ident: `top_k_row_right_mass_sum`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 82,
                        path: `mnist_classifier::connected_component::top_k_row_right_mass_sum`,
                    },
                    ModuleItem {
                        ident: `ConnectedComponentDistribution`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 126,
                        path: `mnist_classifier::connected_component::ConnectedComponentDistribution`,
                    },
                    ModuleItem {
                        ident: `EffHoles`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 127,
                        path: `mnist_classifier::connected_component::EffHoles`,
                    },
                    ModuleItem {
                        ident: `hole_tmpl`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 128,
                        path: `mnist_classifier::connected_component::hole_tmpl`,
                    },
                    ModuleItem {
                        ident: `ConnectedComponent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 129,
                        path: `mnist_classifier::connected_component::ConnectedComponent`,
                    },
                    ModuleItem {
                        ident: `horizontal_extend`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 130,
                        path: `mnist_classifier::connected_component::horizontal_extend`,
                    },
                    ModuleItem {
                        ident: `find_connected_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 131,
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
                        ident: `norm`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 44,
                        path: `mnist_classifier::line_segment_sketch::concave_component::norm`,
                    },
                    ModuleItem {
                        ident: `rel_norm`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 45,
                        path: `mnist_classifier::line_segment_sketch::concave_component::rel_norm`,
                    },
                    ModuleItem {
                        ident: `hausdorff_norm`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 46,
                        path: `mnist_classifier::line_segment_sketch::concave_component::hausdorff_norm`,
                    },
                    ModuleItem {
                        ident: `angle_change`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 47,
                        path: `mnist_classifier::line_segment_sketch::concave_component::angle_change`,
                    },
                    ModuleItem {
                        ident: `bounding_box`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 48,
                        path: `mnist_classifier::line_segment_sketch::concave_component::bounding_box`,
                    },
                    ModuleItem {
                        ident: `relative_bounding_box`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 49,
                        path: `mnist_classifier::line_segment_sketch::concave_component::relative_bounding_box`,
                    },
                    ModuleItem {
                        ident: `line_segment`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 50,
                        path: `mnist_classifier::line_segment_sketch::concave_component::line_segment`,
                    },
                    ModuleItem {
                        ident: `start`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 51,
                        path: `mnist_classifier::line_segment_sketch::concave_component::start`,
                    },
                    ModuleItem {
                        ident: `end`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 52,
                        path: `mnist_classifier::line_segment_sketch::concave_component::end`,
                    },
                    ModuleItem {
                        ident: `displacement`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 53,
                        path: `mnist_classifier::line_segment_sketch::concave_component::displacement`,
                    },
                    ModuleItem {
                        ident: `start_tangent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 54,
                        path: `mnist_classifier::line_segment_sketch::concave_component::start_tangent`,
                    },
                    ModuleItem {
                        ident: `end_tangent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 55,
                        path: `mnist_classifier::line_segment_sketch::concave_component::end_tangent`,
                    },
                    ModuleItem {
                        ident: `ConcaveComponent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 78,
                        path: `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`,
                    },
                    ModuleItem {
                        ident: `find_concave_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 79,
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
                        ident: `displacement`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 15,
                        path: `mnist_classifier::line_segment_sketch::line_segment::displacement`,
                    },
                    ModuleItem {
                        ident: `dist_to_point`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 16,
                        path: `mnist_classifier::line_segment_sketch::line_segment::dist_to_point`,
                    },
                    ModuleItem {
                        ident: `LineSegment`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 18,
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