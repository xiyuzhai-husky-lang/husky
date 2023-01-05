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
                        ident: `digits`,
                        accessibility: PubicUnder(
                            `mnist_classifier`,
                        ),
                        ast_idx: 17,
                        module_path: `mnist_classifier::digits`,
                    },
                    Module {
                        ident: `major`,
                        accessibility: PubicUnder(
                            `mnist_classifier`,
                        ),
                        ast_idx: 18,
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
                        ast_idx: 10,
                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                    },
                    ModuleItem {
                        ident: `EffHoles`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 11,
                        path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                    },
                    ModuleItem {
                        ident: `hole_tmpl`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 12,
                        path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                    },
                    ModuleItem {
                        ident: `ConnectedComponent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 14,
                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                        ast_idx: 144,
                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    },
                    ModuleItem {
                        ident: `Direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 146,
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    },
                    ModuleItem {
                        ident: `get_pixel_pair`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 151,
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_pixel_to_the_left`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 152,
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_pixel_to_the_right`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 153,
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_inward_direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 154,
                        path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_angle_change`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 155,
                        path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_outward_direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 156,
                        path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                    },
                    ModuleItem {
                        ident: `StreakCache`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 157,
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                    },
                    ModuleItem {
                        ident: `get_concave_middle_point`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 158,
                        path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                    },
                    ModuleItem {
                        ident: `find_raw_contours`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 159,
                        path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
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
                        ast_idx: 31,
                        path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    },
                    ModuleItem {
                        ident: `RelativePoint2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 33,
                        path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                    },
                    ModuleItem {
                        ident: `Vector2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 34,
                        path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                        ast_idx: 154,
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                    },
                    Module {
                        ident: `convex_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 155,
                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                    },
                    Module {
                        ident: `convexity`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 156,
                        module_path: `mnist_classifier::line_segment_sketch::convexity`,
                    },
                    Module {
                        ident: `line_segment`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 157,
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                    },
                    ModuleItem {
                        ident: `LineSegmentStroke`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 163,
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    },
                    ModuleItem {
                        ident: `LineSegmentSketch`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 165,
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    },
                    ModuleItem {
                        ident: `go_right`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 167,
                        path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                    },
                    ModuleItem {
                        ident: `go_left`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 168,
                        path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                    },
                    ModuleItem {
                        ident: `extend_end`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 169,
                        path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                    },
                    ModuleItem {
                        ident: `extend_start`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 170,
                        path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                    },
                    ModuleItem {
                        ident: `find_line_segments`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 175,
                        path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
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
                        ast_idx: 7,
                        path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent`, `Struct`),
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
                        ast_idx: 22,
                        path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
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
                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                        ast_idx: 3,
                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    },
                    ModuleItem {
                        ident: `fermi_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 20,
                        path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits`,
                module_symbols: [
                    Module {
                        ident: `zero`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 0,
                        module_path: `mnist_classifier::digits::zero`,
                    },
                    Module {
                        ident: `one`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 1,
                        module_path: `mnist_classifier::digits::one`,
                    },
                    Module {
                        ident: `six`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 2,
                        module_path: `mnist_classifier::digits::six`,
                    },
                    Module {
                        ident: `three`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 3,
                        module_path: `mnist_classifier::digits::three`,
                    },
                    Module {
                        ident: `four`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 4,
                        module_path: `mnist_classifier::digits::four`,
                    },
                    Module {
                        ident: `five`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 5,
                        module_path: `mnist_classifier::digits::five`,
                    },
                    Module {
                        ident: `seven`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 7,
                        module_path: `mnist_classifier::digits::seven`,
                    },
                    Module {
                        ident: `eight`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 8,
                        module_path: `mnist_classifier::digits::eight`,
                    },
                    Module {
                        ident: `nine`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 9,
                        module_path: `mnist_classifier::digits::nine`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::zero`,
                module_symbols: [
                    ModuleItem {
                        ident: `open_one_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                        ast_idx: 25,
                        path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `almost_closed`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                        ast_idx: 26,
                        path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                    },
                    ModuleItem {
                        ident: `is_zero`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                        ast_idx: 27,
                        path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::one`,
                module_symbols: [
                    ModuleItem {
                        ident: `one_fermi_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                        ast_idx: 68,
                        path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_one`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                        ast_idx: 69,
                        path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                    },
                    ModuleItem {
                        ident: `upmost`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                        ast_idx: 70,
                        path: FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                    },
                    ModuleItem {
                        ident: `downmost`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                        ast_idx: 71,
                        path: FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                    },
                    ModuleItem {
                        ident: `hat`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                        ast_idx: 72,
                        path: FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::six`,
                module_symbols: [
                    ModuleItem {
                        ident: `six_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 47,
                        path: FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `six_match_refined1`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 48,
                        path: FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_six`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 49,
                        path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                    },
                    ModuleItem {
                        ident: `upmost`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 58,
                        path: FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                    },
                    ModuleItem {
                        ident: `bottom1`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 59,
                        path: FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::three`,
                module_symbols: [
                    ModuleItem {
                        ident: `three_fermi_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                        ast_idx: 8,
                        path: FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Feature`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::four`,
                module_symbols: [
                    ModuleItem {
                        ident: `left_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 11,
                        path: FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                    },
                    ModuleItem {
                        ident: `left_coordinate_max`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 12,
                        path: FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                    },
                    ModuleItem {
                        ident: `components_max_downwards`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 13,
                        path: FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                    },
                    ModuleItem {
                        ident: `components_max_heights`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 14,
                        path: FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_four`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 15,
                        path: FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::seven`,
                module_symbols: [
                    ModuleItem {
                        ident: `simple_seven_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                        ast_idx: 50,
                        path: FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `simple_leftdown_pattern`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                        ast_idx: 51,
                        path: FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                    },
                    ModuleItem {
                        ident: `special_seven_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                        ast_idx: 52,
                        path: FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `leftupcc_pattern`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                        ast_idx: 53,
                        path: FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                    },
                    ModuleItem {
                        ident: `leftdowncc_pattern`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                        ast_idx: 54,
                        path: FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                    },
                    ModuleItem {
                        ident: `is_seven`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                        ast_idx: 55,
                        path: FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::eight`,
                module_symbols: [
                    ModuleItem {
                        ident: `upper_mouth_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::eight`,
                        ),
                        ast_idx: 16,
                        path: FormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_eight`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::eight`,
                        ),
                        ast_idx: 17,
                        path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::nine`,
                module_symbols: [
                    ModuleItem {
                        ident: `nine_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                        ast_idx: 36,
                        path: FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `nine_match_refine`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                        ast_idx: 37,
                        path: FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_nine`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                        ast_idx: 38,
                        path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
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
                        ast_idx: 19,
                        path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                    },
                    ModuleItem {
                        ident: `major_connected_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 20,
                        path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                    },
                    ModuleItem {
                        ident: `ignored_connected_components_row_span_sum_sum`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 21,
                        path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                    },
                    ModuleItem {
                        ident: `major_raw_contours`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 22,
                        path: FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                    },
                ],
            },
        ],
    },
)