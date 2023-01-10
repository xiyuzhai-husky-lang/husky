Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `mnist_classifier`,
                module_specific_symbols: [
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
                module_specific_symbols: [
                    ModuleItem {
                        ident: `ConnectedComponentDistribution`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 120,
                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                    },
                    ModuleItem {
                        ident: `EffHoles`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 121,
                        path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                    },
                    ModuleItem {
                        ident: `hole_tmpl`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 122,
                        path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                    },
                    ModuleItem {
                        ident: `ConnectedComponent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 123,
                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    },
                    ModuleItem {
                        ident: `horizontal_extend`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 125,
                        path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                    },
                    ModuleItem {
                        ident: `find_connected_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 126,
                        path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::raw_contour`,
                module_specific_symbols: [
                    ModuleItem {
                        ident: `RawContour`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 203,
                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    },
                    ModuleItem {
                        ident: `Direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 205,
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    },
                    ModuleItem {
                        ident: `get_pixel_pair`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 210,
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_pixel_to_the_left`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 211,
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_pixel_to_the_right`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 212,
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_inward_direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 213,
                        path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_angle_change`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 214,
                        path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                    },
                    ModuleItem {
                        ident: `get_outward_direction`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 215,
                        path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                    },
                    ModuleItem {
                        ident: `StreakCache`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 216,
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                    },
                    ModuleItem {
                        ident: `get_concave_middle_point`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 217,
                        path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                    },
                    ModuleItem {
                        ident: `find_raw_contours`,
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 218,
                        path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::geom2d`,
                module_specific_symbols: [
                    ModuleItem {
                        ident: `Point2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 80,
                        path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    },
                    ModuleItem {
                        ident: `RelativePoint2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 82,
                        path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                    },
                    ModuleItem {
                        ident: `Vector2d`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 83,
                        path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                    },
                    ModuleItem {
                        ident: `ClosedRange`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 85,
                        path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                    },
                    ModuleItem {
                        ident: `BoundingBox`,
                        accessibility: Public,
                        ast_idx: 87,
                        path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                    },
                    ModuleItem {
                        ident: `RelativeBoundingBox`,
                        accessibility: PubicUnder(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 90,
                        path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch`,
                module_specific_symbols: [
                    Module {
                        ident: `concave_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 159,
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                    },
                    Module {
                        ident: `convex_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 160,
                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                    },
                    Module {
                        ident: `convexity`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 161,
                        module_path: `mnist_classifier::line_segment_sketch::convexity`,
                    },
                    Module {
                        ident: `line_segment`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 162,
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                    },
                    ModuleItem {
                        ident: `LineSegmentStroke`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 168,
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    },
                    ModuleItem {
                        ident: `LineSegmentSketch`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 170,
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    },
                    ModuleItem {
                        ident: `go_right`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 172,
                        path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                    },
                    ModuleItem {
                        ident: `go_left`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 173,
                        path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                    },
                    ModuleItem {
                        ident: `extend_end`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 174,
                        path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                    },
                    ModuleItem {
                        ident: `extend_start`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 175,
                        path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                    },
                    ModuleItem {
                        ident: `find_line_segments`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 176,
                        path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                module_specific_symbols: [
                    ModuleItem {
                        ident: `ConcaveComponent`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    },
                    ModuleItem {
                        ident: `find_concave_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 75,
                        path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                module_specific_symbols: [
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
                module_specific_symbols: [
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
                module_specific_symbols: [
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
                module_specific_symbols: [
                    ModuleItem {
                        ident: `FermiMatchResult`,
                        accessibility: PubicUnder(
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    },
                    ModuleItem {
                        ident: `fermi_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 24,
                        path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits`,
                module_specific_symbols: [
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
                module_specific_symbols: [
                    ModuleItem {
                        ident: `open_one_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                        ast_idx: 33,
                        path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `almost_closed`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                        ast_idx: 34,
                        path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                    },
                    ModuleItem {
                        ident: `is_zero`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                        ast_idx: 35,
                        path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::one`,
                module_specific_symbols: [
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
                module_specific_symbols: [
                    ModuleItem {
                        ident: `six_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 57,
                        path: FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `six_match_refined1`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 58,
                        path: FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_six`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 59,
                        path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                    },
                    ModuleItem {
                        ident: `upmost`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 60,
                        path: FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                    },
                    ModuleItem {
                        ident: `bottom1`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::six`,
                        ),
                        ast_idx: 61,
                        path: FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::three`,
                module_specific_symbols: [
                    ModuleItem {
                        ident: `three_fermi_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                        ast_idx: 34,
                        path: FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_three`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                        ast_idx: 35,
                        path: FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                    },
                    ModuleItem {
                        ident: `uparc`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                        ast_idx: 36,
                        path: FormPath(`mnist_classifier::digits::three::uparc`, `Function`),
                    },
                    ModuleItem {
                        ident: `downarc`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                        ast_idx: 37,
                        path: FormPath(`mnist_classifier::digits::three::downarc`, `Function`),
                    },
                    ModuleItem {
                        ident: `back`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                        ast_idx: 38,
                        path: FormPath(`mnist_classifier::digits::three::back`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::four`,
                module_specific_symbols: [
                    ModuleItem {
                        ident: `left_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 44,
                        path: FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                    },
                    ModuleItem {
                        ident: `left_coordinate_max`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 45,
                        path: FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                    },
                    ModuleItem {
                        ident: `components_max_downwards`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 46,
                        path: FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                    },
                    ModuleItem {
                        ident: `components_max_heights`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 47,
                        path: FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_four`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 48,
                        path: FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                    },
                    ModuleItem {
                        ident: `displacement_downwards`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 49,
                        path: FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                    },
                    ModuleItem {
                        ident: `cc_box_heights`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::four`,
                        ),
                        ast_idx: 50,
                        path: FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::seven`,
                module_specific_symbols: [
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
                module_specific_symbols: [
                    ModuleItem {
                        ident: `upper_mouth_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::eight`,
                        ),
                        ast_idx: 28,
                        path: FormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_eight`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::eight`,
                        ),
                        ast_idx: 29,
                        path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                    },
                    ModuleItem {
                        ident: `big_mouth`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::eight`,
                        ),
                        ast_idx: 30,
                        path: FormPath(`mnist_classifier::digits::eight::big_mouth`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::nine`,
                module_specific_symbols: [
                    ModuleItem {
                        ident: `nine_match`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                        ast_idx: 44,
                        path: FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
                    },
                    ModuleItem {
                        ident: `nine_match_refine`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                        ast_idx: 45,
                        path: FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                    },
                    ModuleItem {
                        ident: `is_nine`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                        ast_idx: 46,
                        path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                    },
                    ModuleItem {
                        ident: `downmost`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                        ast_idx: 47,
                        path: FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
                    },
                    ModuleItem {
                        ident: `big_cc`,
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                        ast_idx: 48,
                        path: FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::major`,
                module_specific_symbols: [
                    ModuleItem {
                        ident: `connected_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 22,
                        path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                    },
                    ModuleItem {
                        ident: `major_connected_component`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 23,
                        path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                    },
                    ModuleItem {
                        ident: `ignored_connected_components_row_span_sum_sum`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 24,
                        path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                    },
                    ModuleItem {
                        ident: `major_raw_contours`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 25,
                        path: FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                    },
                    ModuleItem {
                        ident: `major_raw_contour`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 26,
                        path: FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                    },
                    ModuleItem {
                        ident: `major_line_segment_sketch`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 27,
                        path: FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                    },
                    ModuleItem {
                        ident: `major_concave_components`,
                        accessibility: PubicUnder(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 28,
                        path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                    },
                ],
            },
        ],
    },
)