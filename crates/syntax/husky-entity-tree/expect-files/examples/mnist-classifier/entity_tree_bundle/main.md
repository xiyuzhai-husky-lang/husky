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
                                        },
                                    ),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            accessibility: PubicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                                        },
                                    ),
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
                                        },
                                    ),
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
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                                },
                                            ),
                                        },
                                    ),
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
                                                },
                                            ),
                                        },
                                    ),
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
                            ident: `ConvexCompoent`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent`, `Struct`),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                                },
                                            ),
                                        },
                                    ),
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
                                                },
                                            ),
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                                },
                                            ),
                                        },
                                    ),
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
                                                },
                                            ),
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: Use(
                                        UseSymbol {
                                            original_symbol: ModuleItem(
                                                ModuleItemSymbol {
                                                    path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                                                },
                                            ),
                                        },
                                    ),
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
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)