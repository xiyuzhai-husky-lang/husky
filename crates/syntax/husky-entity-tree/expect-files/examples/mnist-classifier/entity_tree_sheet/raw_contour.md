Ok(
    EntityTreeSheet {
        module_path: `mnist_classifier::raw_contour`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `RawContour`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 202,
                            ident_token: IdentToken {
                                ident: `RawContour`,
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Direction`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 204,
                            ident_token: IdentToken {
                                ident: `Direction`,
                                token_idx: TokenIdx(
                                    394,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `get_pixel_to_the_left`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 206,
                            ident_token: IdentToken {
                                ident: `get_pixel_to_the_left`,
                                token_idx: TokenIdx(
                                    429,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `get_pixel_to_the_right`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 207,
                            ident_token: IdentToken {
                                ident: `get_pixel_to_the_right`,
                                token_idx: TokenIdx(
                                    450,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `get_inward_direction`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 208,
                            ident_token: IdentToken {
                                ident: `get_inward_direction`,
                                token_idx: TokenIdx(
                                    475,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `get_angle_change`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 209,
                            ident_token: IdentToken {
                                ident: `get_angle_change`,
                                token_idx: TokenIdx(
                                    621,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `get_outward_direction`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 210,
                            ident_token: IdentToken {
                                ident: `get_outward_direction`,
                                token_idx: TokenIdx(
                                    685,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `StreakCache`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 211,
                            ident_token: IdentToken {
                                ident: `StreakCache`,
                                token_idx: TokenIdx(
                                    912,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `get_concave_middle_point`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 212,
                            ident_token: IdentToken {
                                ident: `get_concave_middle_point`,
                                token_idx: TokenIdx(
                                    924,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_raw_contours`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 213,
                            ident_token: IdentToken {
                                ident: `find_raw_contours`,
                                token_idx: TokenIdx(
                                    990,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `connected_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::connected_component`,
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 11,
                                    ident_token: IdentToken {
                                        ident: `connected_component`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::connected_component`,
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `raw_contour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::raw_contour`,
                                    visibility: Visibility::PubUnder(
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
                            path: EntityPath::Module(
                                `mnist_classifier::raw_contour`,
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `geom2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::geom2d`,
                                    visibility: Visibility::PubUnder(
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
                            path: EntityPath::Module(
                                `mnist_classifier::geom2d`,
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `line_segment_sketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch`,
                                    visibility: Visibility::PubUnder(
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
                            path: EntityPath::Module(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::fermi`,
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 15,
                                    ident_token: IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::fermi`,
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `digits`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits`,
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 16,
                                    ident_token: IdentToken {
                                        ident: `digits`,
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::digits`,
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::major`,
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 17,
                                    ident_token: IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            13,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::major`,
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `MnistLabel`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 10,
                                            ident_token: IdentToken {
                                                ident: `MnistLabel`,
                                                token_idx: TokenIdx(
                                                    2,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 24,
                                    use_expr_idx: 18,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Point2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 198,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativePoint2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 198,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Vector2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 198,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ClosedRange`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 198,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BoundingBox`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 198,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativeBoundingBox`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 198,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponentDistribution`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 120,
                                    ident_token: IdentToken {
                                        ident: `ConnectedComponentDistribution`,
                                        token_idx: TokenIdx(
                                            12,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `EffHoles`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 121,
                                    ident_token: IdentToken {
                                        ident: `EffHoles`,
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponent`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 123,
                                    ident_token: IdentToken {
                                        ident: `ConnectedComponent`,
                                        token_idx: TokenIdx(
                                            71,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_connected_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 126,
                                    ident_token: IdentToken {
                                        ident: `find_connected_components`,
                                        token_idx: TokenIdx(
                                            646,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 200,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 171,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentSketch`,
                                        token_idx: TokenIdx(
                                            161,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 200,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                            visibility: Visibility::Pub,
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
                                    visibility: Visibility::Pub,
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 200,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_concave_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                            ),
                                            visibility: Visibility::Pub,
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
                                            FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 200,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                            visibility: Visibility::Pub,
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
                                    visibility: Visibility::Pub,
                                    ast_idx: 165,
                                    use_expr_idx: 4,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 200,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `connected_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 19,
                                            ident_token: IdentToken {
                                                ident: `connected_components`,
                                                token_idx: TokenIdx(
                                                    6,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_connected_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 20,
                                            ident_token: IdentToken {
                                                ident: `major_connected_component`,
                                                token_idx: TokenIdx(
                                                    20,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ignored_connected_components_row_span_sum_sum`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 21,
                                            ident_token: IdentToken {
                                                ident: `ignored_connected_components_row_span_sum_sum`,
                                                token_idx: TokenIdx(
                                                    72,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contours`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 22,
                                            ident_token: IdentToken {
                                                ident: `major_raw_contours`,
                                                token_idx: TokenIdx(
                                                    107,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 23,
                                            ident_token: IdentToken {
                                                ident: `major_raw_contour`,
                                                token_idx: TokenIdx(
                                                    120,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_line_segment_sketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 24,
                                            ident_token: IdentToken {
                                                ident: `major_line_segment_sketch`,
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_concave_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 25,
                                            ident_token: IdentToken {
                                                ident: `major_concave_components`,
                                                token_idx: TokenIdx(
                                                    145,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_one`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::ModuleItem(
                                                ModuleItemSymbol {
                                                    path: ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                    visibility: Visibility::Pub,
                                                    ast_idx: 63,
                                                    ident_token: IdentToken {
                                                        ident: `is_one`,
                                                        token_idx: TokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                },
                                            ),
                                            path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                ),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 10,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 3,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `FermiMatchResult`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 22,
                                            ident_token: IdentToken {
                                                ident: `FermiMatchResult`,
                                                token_idx: TokenIdx(
                                                    6,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 20,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi_match`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 24,
                                            ident_token: IdentToken {
                                                ident: `fermi_match`,
                                                token_idx: TokenIdx(
                                                    150,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 20,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 9,
                        },
                    ),
                },
            ],
        ),
        impl_blocks: [
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 203,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            38,
                        ),
                    },
                    ty_expr: 1,
                    body: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                27..34,
                            ),
                        },
                    ),
                },
            ),
        ],
        use_expr_rules: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 198,
                    use_expr_idx: 2,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                    ast_idx: 199,
                    use_expr_idx: 5,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                    ast_idx: 200,
                    use_expr_idx: 8,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                    ast_idx: 201,
                    use_expr_idx: 10,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            9..10,
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
                    ast_idx: 198,
                    use_expr_idx: 1,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `geom2d`,
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
                                path: `mnist_classifier::geom2d`,
                                visibility: Visibility::PubUnder(
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
                UseExprRule {
                    ast_idx: 199,
                    use_expr_idx: 4,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `connected_component`,
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
                                path: `mnist_classifier::connected_component`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 11,
                                ident_token: IdentToken {
                                    ident: `connected_component`,
                                    token_idx: TokenIdx(
                                        1,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 200,
                    use_expr_idx: 7,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment_sketch`,
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
                                path: `mnist_classifier::line_segment_sketch`,
                                visibility: Visibility::PubUnder(
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
            ],
        ),
        use_all_rules: UseAllRules(
            [
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier`,
                    },
                    ast_idx: 201,
                    use_expr_idx: 9,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    progress: 29,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::geom2d`,
                    },
                    ast_idx: 198,
                    use_expr_idx: 0,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    progress: 6,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::connected_component`,
                    },
                    ast_idx: 199,
                    use_expr_idx: 3,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    progress: 31,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::line_segment_sketch`,
                    },
                    ast_idx: 200,
                    use_expr_idx: 6,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    progress: 21,
                },
            ],
        ),
        errors: [],
    },
)