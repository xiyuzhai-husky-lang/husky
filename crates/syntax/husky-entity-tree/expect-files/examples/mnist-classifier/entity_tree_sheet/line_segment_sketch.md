Ok(
    EntityTreeSheet {
        module_path: `mnist_classifier::line_segment_sketch`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `concave_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::concave_component`,
                            visibility: Visibility::PubUnder(
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
                },
                EntitySymbolEntry {
                    ident: `convex_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::convex_component`,
                            visibility: Visibility::PubUnder(
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
                },
                EntitySymbolEntry {
                    ident: `convexity`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::convexity`,
                            visibility: Visibility::PubUnder(
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
                },
                EntitySymbolEntry {
                    ident: `line_segment`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::line_segment`,
                            visibility: Visibility::PubUnder(
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
                },
                EntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
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
                },
                EntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
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
                },
                EntitySymbolEntry {
                    ident: `go_right`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 173,
                            ident_token: IdentToken {
                                ident: `go_right`,
                                token_idx: TokenIdx(
                                    348,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `go_left`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 174,
                            ident_token: IdentToken {
                                ident: `go_left`,
                                token_idx: TokenIdx(
                                    445,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `extend_end`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 175,
                            ident_token: IdentToken {
                                ident: `extend_end`,
                                token_idx: TokenIdx(
                                    542,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `extend_start`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 176,
                            ident_token: IdentToken {
                                ident: `extend_start`,
                                token_idx: TokenIdx(
                                    775,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_line_segments`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 177,
                            ident_token: IdentToken {
                                ident: `find_line_segments`,
                                token_idx: TokenIdx(
                                    1041,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::Use(
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
                },
                EntitySymbolEntry {
                    ident: `find_concave_components`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::Use(
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
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::Use(
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
                },
                EntitySymbolEntry {
                    ident: `RawContour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 199,
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 167,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Point2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
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
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 166,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativePoint2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
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
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 166,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Vector2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
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
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 166,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ClosedRange`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
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
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 166,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BoundingBox`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
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
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 166,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativeBoundingBox`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
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
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 166,
                            use_expr_idx: 6,
                        },
                    ),
                },
            ],
        ),
        impl_blocks: [
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 170,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            81,
                        ),
                    },
                    ty_expr: 6,
                    body: ArenaIdxRange(
                        3..7,
                    ),
                },
            ),
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 172,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            175,
                        ),
                    },
                    ty_expr: 7,
                    body: ArenaIdxRange(
                        21..26,
                    ),
                },
            ),
        ],
        use_expr_rules: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 163,
                    use_expr_idx: 1,
                    visibility: Visibility::Pub,
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::line_segment_sketch::concave_component`,
                                visibility: Visibility::PubUnder(
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
                    },
                },
                UseExprRule {
                    ast_idx: 164,
                    use_expr_idx: 3,
                    visibility: Visibility::Pub,
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `convex_component`,
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            2..3,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::line_segment_sketch::convex_component`,
                                visibility: Visibility::PubUnder(
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
                    },
                },
                UseExprRule {
                    ast_idx: 165,
                    use_expr_idx: 5,
                    visibility: Visibility::Pub,
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment`,
                                token_idx: TokenIdx(
                                    20,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            4..5,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::line_segment_sketch::line_segment`,
                                visibility: Visibility::PubUnder(
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
                    },
                },
                UseExprRule {
                    ast_idx: 166,
                    use_expr_idx: 8,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    24,
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
                    ast_idx: 167,
                    use_expr_idx: 11,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    30,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            10..11,
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
                    ast_idx: 168,
                    use_expr_idx: 13,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment`,
                                token_idx: TokenIdx(
                                    36,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            12..13,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::line_segment_sketch::line_segment`,
                                visibility: Visibility::PubUnder(
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
                    },
                },
                UseExprRule {
                    ast_idx: 166,
                    use_expr_idx: 7,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `geom2d`,
                                token_idx: TokenIdx(
                                    26,
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
                    ast_idx: 167,
                    use_expr_idx: 10,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    32,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            9..10,
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
                    },
                },
                UseExprRule {
                    ast_idx: 167,
                    use_expr_idx: 9,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `RawContour`,
                            token_idx: TokenIdx(
                                34,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::raw_contour`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                                visibility: Visibility::Pub,
                                ast_idx: 199,
                                ident_token: IdentToken {
                                    ident: `RawContour`,
                                    token_idx: TokenIdx(
                                        24,
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
                        path: `mnist_classifier::line_segment_sketch::concave_component`,
                    },
                    ast_idx: 163,
                    use_expr_idx: 0,
                    visibility: Visibility::Pub,
                    progress: 39,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::line_segment_sketch::convex_component`,
                    },
                    ast_idx: 164,
                    use_expr_idx: 2,
                    visibility: Visibility::Pub,
                    progress: 22,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::line_segment_sketch::line_segment`,
                    },
                    ast_idx: 165,
                    use_expr_idx: 4,
                    visibility: Visibility::Pub,
                    progress: 7,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::line_segment_sketch::line_segment`,
                    },
                    ast_idx: 168,
                    use_expr_idx: 12,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    progress: 7,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::geom2d`,
                    },
                    ast_idx: 166,
                    use_expr_idx: 6,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    progress: 6,
                },
            ],
        ),
        errors: [],
    },
)