Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `mnist_classifier`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::connected_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 12,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::raw_contour`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 13,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::geom2d`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 14,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 15,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::fermi`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 16,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 17,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::major`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::raw_contour`,
                                            ),
                                            ast_idx: 218,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 23,
                                    use_expr_idx: 8,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                                    accessibility: Public,
                                                    ast_idx: 69,
                                                },
                                            ),
                                            path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                            accessibility: Public,
                                            ast_idx: 10,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 6,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    accessibility: Public,
                                                    ast_idx: 73,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 165,
                                            use_expr_idx: 6,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 24,
                                    use_expr_idx: 10,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::connected_component`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `ConnectedComponentDistribution`,
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::connected_component`,
                                    ),
                                    ast_idx: 120,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `EffHoles`,
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::connected_component`,
                                    ),
                                    ast_idx: 121,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `hole_tmpl`,
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::connected_component`,
                                    ),
                                    ast_idx: 122,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConnectedComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::connected_component`,
                                    ),
                                    ast_idx: 123,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `horizontal_extend`,
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::connected_component`,
                                    ),
                                    ast_idx: 125,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_connected_components`,
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::connected_component`,
                                    ),
                                    ast_idx: 126,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::raw_contour`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 203,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Direction`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 205,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_pixel_pair`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 210,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_pixel_to_the_left`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 211,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_pixel_to_the_right`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 212,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_inward_direction`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 213,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_angle_change`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 214,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_outward_direction`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 215,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `StreakCache`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 216,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_concave_middle_point`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 217,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 218,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 87,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 199,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    accessibility: Public,
                                                    ast_idx: 73,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 165,
                                            use_expr_idx: 6,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    ast_idx: 201,
                                    use_expr_idx: 6,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::geom2d`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            accessibility: PubicUnder(
                                `mnist_classifier::geom2d`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    ast_idx: 80,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            accessibility: PubicUnder(
                                `mnist_classifier::geom2d`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    ast_idx: 82,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            accessibility: PubicUnder(
                                `mnist_classifier::geom2d`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    ast_idx: 83,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            accessibility: PubicUnder(
                                `mnist_classifier::geom2d`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    ast_idx: 85,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    accessibility: Public,
                                    ast_idx: 87,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            accessibility: PubicUnder(
                                `mnist_classifier::geom2d`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    ast_idx: 90,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `concave_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 159,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch::convex_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 160,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch::convexity`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 161,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch::line_segment`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 162,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 168,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 170,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 172,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 173,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 174,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 175,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 176,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 73,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: Public,
                                    ast_idx: 165,
                                    use_expr_idx: 6,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::raw_contour`,
                                            ),
                                            ast_idx: 203,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 164,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 87,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: Public,
                                    ast_idx: 73,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 75,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `concave_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 159,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::convex_component`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 160,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::convex_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::convexity`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 161,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::convexity`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::line_segment`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 162,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::line_segment`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 168,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 170,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 172,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 173,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 174,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 175,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 176,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::raw_contour`,
                                                    ),
                                                    ast_idx: 203,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 164,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    accessibility: Public,
                                                    ast_idx: 87,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 163,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    ast_idx: 69,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `ConvexComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `concave_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 159,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::convex_component`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 160,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::convex_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::convexity`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 161,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::convexity`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::line_segment`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 162,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::line_segment`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 168,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 170,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 172,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 173,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 174,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 175,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 176,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    accessibility: Public,
                                                    ast_idx: 73,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 165,
                                            use_expr_idx: 6,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::raw_contour`,
                                                    ),
                                                    ast_idx: 203,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 164,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    accessibility: Public,
                                                    ast_idx: 87,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 163,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    ast_idx: 2,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::convexity`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `is_convex`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 22,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `concave_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 159,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::convex_component`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 160,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::convex_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::convexity`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 161,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::convexity`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch::line_segment`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 162,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch::line_segment`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 168,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 170,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 172,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 173,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 174,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 175,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 176,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    accessibility: Public,
                                                    ast_idx: 73,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 165,
                                            use_expr_idx: 6,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::raw_contour`,
                                                    ),
                                                    ast_idx: 203,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 164,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    accessibility: Public,
                                                    ast_idx: 87,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            ast_idx: 163,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::line_segment`,
                                    ),
                                    ast_idx: 16,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 87,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::line_segment`,
                                    ),
                                    ast_idx: 15,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::fermi`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `FermiMatchResult`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 22,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 24,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::connected_component`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 12,
                                        },
                                    ),
                                    path: `mnist_classifier::connected_component`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::raw_contour`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 13,
                                        },
                                    ),
                                    path: `mnist_classifier::raw_contour`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::geom2d`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 14,
                                        },
                                    ),
                                    path: `mnist_classifier::geom2d`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::line_segment_sketch`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 15,
                                        },
                                    ),
                                    path: `mnist_classifier::line_segment_sketch`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::fermi`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 16,
                                        },
                                    ),
                                    path: `mnist_classifier::fermi`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 17,
                                        },
                                    ),
                                    path: `mnist_classifier::digits`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::major`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 18,
                                        },
                                    ),
                                    path: `mnist_classifier::major`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::raw_contour`,
                                                    ),
                                                    ast_idx: 218,
                                                },
                                            ),
                                            path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 23,
                                            use_expr_idx: 8,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: Use(
                                                UseSymbol {
                                                    original_symbol: ModuleItem(
                                                        ModuleItemSymbol {
                                                            path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                                            accessibility: Public,
                                                            ast_idx: 69,
                                                        },
                                                    ),
                                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                                    accessibility: Public,
                                                    ast_idx: 10,
                                                    use_expr_idx: 0,
                                                },
                                            ),
                                            path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 22,
                                            use_expr_idx: 6,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: Use(
                                                UseSymbol {
                                                    original_symbol: ModuleItem(
                                                        ModuleItemSymbol {
                                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            accessibility: Public,
                                                            ast_idx: 73,
                                                        },
                                                    ),
                                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    accessibility: Public,
                                                    ast_idx: 165,
                                                    use_expr_idx: 6,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier`,
                                            ),
                                            ast_idx: 24,
                                            use_expr_idx: 10,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `zero`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::zero`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `one`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::one`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `six`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::six`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 2,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `three`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::three`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `four`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::four`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 4,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `five`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::five`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 5,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `six`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::six`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 6,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `seven`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::seven`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 7,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `eight`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::eight`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 8,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `nine`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits::nine`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 9,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                            accessibility: Public,
                                            ast_idx: 69,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                    accessibility: Public,
                                    ast_idx: 10,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_six`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits::six`,
                                            ),
                                            ast_idx: 59,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 11,
                                    use_expr_idx: 2,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_zero`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits::zero`,
                                            ),
                                            ast_idx: 35,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 12,
                                    use_expr_idx: 4,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_three`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits::three`,
                                            ),
                                            ast_idx: 35,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 14,
                                    use_expr_idx: 8,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_seven`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits::seven`,
                                            ),
                                            ast_idx: 52,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 17,
                                    use_expr_idx: 14,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_eight`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits::eight`,
                                            ),
                                            ast_idx: 29,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 16,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_nine`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits::nine`,
                                            ),
                                            ast_idx: 46,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 18,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::zero`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `open_one_match`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::zero`,
                                    ),
                                    ast_idx: 33,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `almost_closed`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::zero`,
                                    ),
                                    ast_idx: 34,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_zero`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::zero`,
                                    ),
                                    ast_idx: 35,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 73,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::zero`,
                                    ),
                                    ast_idx: 31,
                                    use_expr_idx: 25,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::one`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `one_fermi_match`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::one`,
                                    ),
                                    ast_idx: 68,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                    accessibility: Public,
                                    ast_idx: 69,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `upmost`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::one`,
                                    ),
                                    ast_idx: 70,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `downmost`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::one`,
                                    ),
                                    ast_idx: 71,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `hat`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::one`,
                                    ),
                                    ast_idx: 72,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 73,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::one`,
                                    ),
                                    ast_idx: 67,
                                    use_expr_idx: 25,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::six`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `six_match`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::six`,
                                    ),
                                    ast_idx: 57,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `six_match_refined1`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::six`,
                                    ),
                                    ast_idx: 58,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_six`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::six`,
                                    ),
                                    ast_idx: 59,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `upmost`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::six`,
                                    ),
                                    ast_idx: 60,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `bottom1`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::six`,
                                    ),
                                    ast_idx: 61,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 73,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::six`,
                                    ),
                                    ast_idx: 55,
                                    use_expr_idx: 25,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::three`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `three_fermi_match`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::three`,
                                    ),
                                    ast_idx: 34,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_three`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::three`,
                                    ),
                                    ast_idx: 35,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `uparc`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::three::uparc`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::three`,
                                    ),
                                    ast_idx: 36,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `downarc`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::three::downarc`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::three`,
                                    ),
                                    ast_idx: 37,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `back`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::three::back`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::three`,
                                    ),
                                    ast_idx: 38,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 73,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::three`,
                                    ),
                                    ast_idx: 33,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::four`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `left_components`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::four`,
                                    ),
                                    ast_idx: 44,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `left_coordinate_max`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::four`,
                                    ),
                                    ast_idx: 45,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `components_max_downwards`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::four`,
                                    ),
                                    ast_idx: 46,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `components_max_heights`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::four`,
                                    ),
                                    ast_idx: 47,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_four`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::four`,
                                    ),
                                    ast_idx: 48,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `displacement_downwards`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::four`,
                                    ),
                                    ast_idx: 49,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `cc_box_heights`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::four`,
                                    ),
                                    ast_idx: 50,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 73,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::four`,
                                    ),
                                    ast_idx: 43,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::five`,
                module_specific_symbols: EntitySymbolTable(
                    [],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::seven`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `simple_seven_match`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 47,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `simple_leftdown_pattern`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 48,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `special_seven_match`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 49,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `leftupcc_pattern`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 50,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `leftdowncc_pattern`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 51,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_seven`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 52,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::major`,
                                            ),
                                            ast_idx: 28,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 40,
                                    use_expr_idx: 6,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::major`,
                                            ),
                                            ast_idx: 23,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 41,
                                    use_expr_idx: 9,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `zero`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits::zero`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: `mnist_classifier::digits::zero`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `one`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits::one`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 1,
                                        },
                                    ),
                                    path: `mnist_classifier::digits::one`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `six`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits::six`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 2,
                                        },
                                    ),
                                    path: `mnist_classifier::digits::six`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `three`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits::three`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 3,
                                        },
                                    ),
                                    path: `mnist_classifier::digits::three`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `four`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits::four`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 4,
                                        },
                                    ),
                                    path: `mnist_classifier::digits::four`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `five`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits::five`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 5,
                                        },
                                    ),
                                    path: `mnist_classifier::digits::five`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `seven`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits::seven`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 7,
                                        },
                                    ),
                                    path: `mnist_classifier::digits::seven`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `eight`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits::eight`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 8,
                                        },
                                    ),
                                    path: `mnist_classifier::digits::eight`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `nine`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Submodule(
                                        SubmoduleSymbol {
                                            path: `mnist_classifier::digits::nine`,
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 9,
                                        },
                                    ),
                                    path: `mnist_classifier::digits::nine`,
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                                    accessibility: Public,
                                                    ast_idx: 69,
                                                },
                                            ),
                                            path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                            accessibility: Public,
                                            ast_idx: 10,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_six`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::digits::six`,
                                                    ),
                                                    ast_idx: 59,
                                                },
                                            ),
                                            path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 11,
                                            use_expr_idx: 2,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_zero`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::digits::zero`,
                                                    ),
                                                    ast_idx: 35,
                                                },
                                            ),
                                            path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 12,
                                            use_expr_idx: 4,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_three`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::digits::three`,
                                                    ),
                                                    ast_idx: 35,
                                                },
                                            ),
                                            path: FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 14,
                                            use_expr_idx: 8,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_eight`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::digits::eight`,
                                                    ),
                                                    ast_idx: 29,
                                                },
                                            ),
                                            path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 18,
                                            use_expr_idx: 16,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_nine`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::digits::nine`,
                                                    ),
                                                    ast_idx: 46,
                                                },
                                            ),
                                            path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::digits`,
                                            ),
                                            ast_idx: 19,
                                            use_expr_idx: 18,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 46,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 73,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::seven`,
                                    ),
                                    ast_idx: 45,
                                    use_expr_idx: 23,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::eight`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `upper_mouth_match`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::eight`,
                                    ),
                                    ast_idx: 28,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_eight`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::eight`,
                                    ),
                                    ast_idx: 29,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `big_mouth`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::eight::big_mouth`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::eight`,
                                    ),
                                    ast_idx: 30,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::major`,
                                            ),
                                            ast_idx: 28,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::eight`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 6,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            accessibility: PubicUnder(
                                                `mnist_classifier::major`,
                                            ),
                                            ast_idx: 23,
                                        },
                                    ),
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::eight`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 9,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 73,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::eight`,
                                    ),
                                    ast_idx: 25,
                                    use_expr_idx: 29,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::nine`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `nine_match`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::nine`,
                                    ),
                                    ast_idx: 44,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `nine_match_refine`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::nine`,
                                    ),
                                    ast_idx: 45,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_nine`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::nine`,
                                    ),
                                    ast_idx: 46,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `downmost`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::nine`,
                                    ),
                                    ast_idx: 47,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `big_cc`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::nine`,
                                    ),
                                    ast_idx: 48,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 73,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::digits::nine`,
                                    ),
                                    ast_idx: 41,
                                    use_expr_idx: 27,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::major`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            accessibility: PubicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::major`,
                                    ),
                                    ast_idx: 22,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::major`,
                                    ),
                                    ast_idx: 23,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            accessibility: PubicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::major`,
                                    ),
                                    ast_idx: 24,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            accessibility: PubicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::major`,
                                    ),
                                    ast_idx: 25,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            accessibility: PubicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::major`,
                                    ),
                                    ast_idx: 26,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            accessibility: PubicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::major`,
                                    ),
                                    ast_idx: 27,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            accessibility: PubicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::major`,
                                    ),
                                    ast_idx: 28,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    accessibility: Public,
                                                    ast_idx: 73,
                                                },
                                            ),
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            accessibility: Public,
                                            ast_idx: 165,
                                            use_expr_idx: 6,
                                        },
                                    ),
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::major`,
                                    ),
                                    ast_idx: 20,
                                    use_expr_idx: 6,
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)