Ok(
    EntityTreeSheet {
        module_path: `mnist_classifier::line_segment_sketch::convexity`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `is_convex`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 22,
                            ident_token: IdentToken {
                                ident: `is_convex`,
                                token_idx: TokenIdx(
                                    20,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `concave_component`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 159,
                                    ident_token: IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `convex_component`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch::convex_component`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 160,
                                    ident_token: IdentToken {
                                        ident: `convex_component`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `convexity`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch::convexity`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 161,
                                    ident_token: IdentToken {
                                        ident: `convexity`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `line_segment`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch::line_segment`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 162,
                                    ident_token: IdentToken {
                                        ident: `line_segment`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 169,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentStroke`,
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 172,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentSketch`,
                                        token_idx: TokenIdx(
                                            169,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `go_right`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 174,
                                    ident_token: IdentToken {
                                        ident: `go_right`,
                                        token_idx: TokenIdx(
                                            356,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `go_left`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 175,
                                    ident_token: IdentToken {
                                        ident: `go_left`,
                                        token_idx: TokenIdx(
                                            453,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `extend_end`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 176,
                                    ident_token: IdentToken {
                                        ident: `extend_end`,
                                        token_idx: TokenIdx(
                                            550,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `extend_start`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 177,
                                    ident_token: IdentToken {
                                        ident: `extend_start`,
                                        token_idx: TokenIdx(
                                            783,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_line_segments`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 178,
                                    ident_token: IdentToken {
                                        ident: `find_line_segments`,
                                        token_idx: TokenIdx(
                                            1049,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 74,
                                            ident_token: IdentToken {
                                                ident: `ConcaveComponent`,
                                                token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_concave_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 76,
                                            ident_token: IdentToken {
                                                ident: `find_concave_components`,
                                                token_idx: TokenIdx(
                                                    522,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 16,
                                            ident_token: IdentToken {
                                                ident: `LineSegment`,
                                                token_idx: TokenIdx(
                                                    8,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 165,
                                    use_expr_idx: 4,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RawContour`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 203,
                                            ident_token: IdentToken {
                                                ident: `RawContour`,
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 167,
                                    use_expr_idx: 9,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Point2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 78,
                                            ident_token: IdentToken {
                                                ident: `Point2d`,
                                                token_idx: TokenIdx(
                                                    2,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 166,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativePoint2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 80,
                                            ident_token: IdentToken {
                                                ident: `RelativePoint2d`,
                                                token_idx: TokenIdx(
                                                    144,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 166,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Vector2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 81,
                                            ident_token: IdentToken {
                                                ident: `Vector2d`,
                                                token_idx: TokenIdx(
                                                    157,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 166,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ClosedRange`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 83,
                                            ident_token: IdentToken {
                                                ident: `ClosedRange`,
                                                token_idx: TokenIdx(
                                                    488,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 166,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BoundingBox`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 85,
                                            ident_token: IdentToken {
                                                ident: `BoundingBox`,
                                                token_idx: TokenIdx(
                                                    596,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 166,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativeBoundingBox`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 87,
                                            ident_token: IdentToken {
                                                ident: `RelativeBoundingBox`,
                                                token_idx: TokenIdx(
                                                    732,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 166,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_raw_contours`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 214,
                                    ident_token: IdentToken {
                                        ident: `find_raw_contours`,
                                        token_idx: TokenIdx(
                                            991,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            ast_idx: 20,
                            use_expr_idx: 3,
                        },
                    ),
                },
            ],
        ),
        impl_blocks: [],
        use_expr_rules: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 2,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            1..2,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 20,
                    use_expr_idx: 5,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            4..5,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 21,
                    use_expr_idx: 8,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            7..8,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::line_segment_sketch`,
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 14,
                                ident_token: IdentToken {
                                    ident: `line_segment_sketch`,
                                    token_idx: TokenIdx(
                                        7,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 20,
                    use_expr_idx: 4,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            3..4,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::raw_contour`,
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 12,
                                ident_token: IdentToken {
                                    ident: `raw_contour`,
                                    token_idx: TokenIdx(
                                        3,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 21,
                    use_expr_idx: 7,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `geom2d`,
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            6..7,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::geom2d`,
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 13,
                                ident_token: IdentToken {
                                    ident: `geom2d`,
                                    token_idx: TokenIdx(
                                        5,
                                    ),
                                },
                            },
                        ),
                    },
                },
            ],
        ),
        use_all_rules: UseAllRules(
            [
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::line_segment_sketch`,
                    },
                    ast_idx: 19,
                    use_expr_idx: 0,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    progress: 21,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::raw_contour`,
                    },
                    ast_idx: 20,
                    use_expr_idx: 3,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    progress: 44,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::geom2d`,
                    },
                    ast_idx: 21,
                    use_expr_idx: 6,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                    progress: 6,
                },
            ],
        ),
        errors: [],
    },
)