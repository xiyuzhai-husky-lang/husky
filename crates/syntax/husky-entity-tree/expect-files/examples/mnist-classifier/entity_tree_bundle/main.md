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
                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `EffHoles`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 122,
                        path: TypePath(`mnist_classifier::connected_component::EffHoles, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `hole_tmpl`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 123,
                        path: FormPath(`mnist_classifier::connected_component::hole_tmpl, Function`),
                    },
                    ModuleItem {
                        ident: `ConnectedComponent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 124,
                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponent, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `horizontal_extend`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 125,
                        path: FormPath(`mnist_classifier::connected_component::horizontal_extend, Function`),
                    },
                    ModuleItem {
                        ident: `find_connected_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 126,
                        path: FormPath(`mnist_classifier::connected_component::find_connected_components, Function`),
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
                        path: TypePath(`mnist_classifier::raw_contour::RawContour, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `Direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 219,
                        path: TypePath(`mnist_classifier::raw_contour::Direction, Enum`),
                    },
                    ModuleItem {
                        ident: `get_pixel_pair`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 224,
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair, Function`),
                    },
                    ModuleItem {
                        ident: `get_pixel_to_the_left`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 225,
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left, Function`),
                    },
                    ModuleItem {
                        ident: `get_pixel_to_the_right`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 226,
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right, Function`),
                    },
                    ModuleItem {
                        ident: `get_inward_direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 227,
                        path: FormPath(`mnist_classifier::raw_contour::get_inward_direction, Function`),
                    },
                    ModuleItem {
                        ident: `get_angle_change`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 228,
                        path: FormPath(`mnist_classifier::raw_contour::get_angle_change, Function`),
                    },
                    ModuleItem {
                        ident: `get_outward_direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 229,
                        path: FormPath(`mnist_classifier::raw_contour::get_outward_direction, Function`),
                    },
                    ModuleItem {
                        ident: `StreakCache`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 230,
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `get_concave_middle_point`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 231,
                        path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point, Function`),
                    },
                    ModuleItem {
                        ident: `find_raw_contours`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 232,
                        path: FormPath(`mnist_classifier::raw_contour::find_raw_contours, Function`),
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
                        ast_idx: 85,
                        path: TypePath(`mnist_classifier::geom2d::Point2d, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `RelativePoint2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 87,
                        path: TypePath(`mnist_classifier::geom2d::RelativePoint2d, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `Vector2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 88,
                        path: TypePath(`mnist_classifier::geom2d::Vector2d, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `ClosedRange`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 90,
                        path: TypePath(`mnist_classifier::geom2d::ClosedRange, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `BoundingBox`,
                        accessibility: Public,
                        ast_idx: 92,
                        path: TypePath(`mnist_classifier::geom2d::BoundingBox, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `RelativeBoundingBox`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 95,
                        path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox, PropsStruct`),
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
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `LineSegmentSketch`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 171,
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `go_right`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 173,
                        path: FormPath(`mnist_classifier::line_segment_sketch::go_right, Function`),
                    },
                    ModuleItem {
                        ident: `go_left`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 174,
                        path: FormPath(`mnist_classifier::line_segment_sketch::go_left, Function`),
                    },
                    ModuleItem {
                        ident: `extend_end`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 175,
                        path: FormPath(`mnist_classifier::line_segment_sketch::extend_end, Function`),
                    },
                    ModuleItem {
                        ident: `extend_start`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 176,
                        path: FormPath(`mnist_classifier::line_segment_sketch::extend_start, Function`),
                    },
                    ModuleItem {
                        ident: `find_line_segments`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 177,
                        path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments, Function`),
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
                        path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `find_concave_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 78,
                        path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components, Function`),
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
                        path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent, PropsStruct`),
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
                        path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex, Function`),
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
                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment, PropsStruct`),
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
                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult, PropsStruct`),
                    },
                    ModuleItem {
                        ident: `fermi_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 26,
                        path: FormPath(`mnist_classifier::fermi::fermi_match, Function`),
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
                        path: FormPath(`mnist_classifier::major::connected_components, Feature`),
                    },
                    ModuleItem {
                        ident: `major_connected_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 23,
                        path: FormPath(`mnist_classifier::major::major_connected_component, Feature`),
                    },
                    ModuleItem {
                        ident: `ignored_connected_components_row_span_sum_sum`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 24,
                        path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum, Feature`),
                    },
                    ModuleItem {
                        ident: `major_raw_contours`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 25,
                        path: FormPath(`mnist_classifier::major::major_raw_contours, Feature`),
                    },
                    ModuleItem {
                        ident: `major_raw_contour`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 27,
                        path: FormPath(`mnist_classifier::major::major_raw_contour, Feature`),
                    },
                    ModuleItem {
                        ident: `major_line_segment_sketch`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 28,
                        path: FormPath(`mnist_classifier::major::major_line_segment_sketch, Feature`),
                    },
                    ModuleItem {
                        ident: `major_concave_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 29,
                        path: FormPath(`mnist_classifier::major::major_concave_components, Feature`),
                    },
                ],
            },
        ],
    },
)