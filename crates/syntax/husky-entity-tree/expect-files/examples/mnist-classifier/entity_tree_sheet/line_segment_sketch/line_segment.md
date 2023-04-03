Ok(
    EntityTreeSheet {
        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
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
                },
                EntitySymbolEntry {
                    ident: `Point2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            ast_idx: 15,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativePoint2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            ast_idx: 15,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Vector2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            ast_idx: 15,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ClosedRange`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            ast_idx: 15,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BoundingBox`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            ast_idx: 15,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativeBoundingBox`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            ast_idx: 15,
                            use_expr_idx: 0,
                        },
                    ),
                },
            ],
        ),
        impl_blocks: [
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 17,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            19,
                        ),
                    },
                    ty_expr: 10,
                    body: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                13..15,
                            ),
                        },
                    ),
                },
            ),
        ],
        use_expr_rules: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 15,
                    use_expr_idx: 2,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                    ast_idx: 15,
                    use_expr_idx: 1,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
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
            ],
        ),
        use_all_rules: UseAllRules(
            [
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::geom2d`,
                    },
                    ast_idx: 15,
                    use_expr_idx: 0,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
                    ),
                    progress: 6,
                },
            ],
        ),
        errors: [],
    },
)