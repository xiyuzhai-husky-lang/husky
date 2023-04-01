Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `concave_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
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
                NativeEntitySymbolEntry {
                    ident: `convex_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
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
                NativeEntitySymbolEntry {
                    ident: `convexity`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
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
                NativeEntitySymbolEntry {
                    ident: `line_segment`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
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
                NativeEntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Visibility::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
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
                NativeEntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Visibility::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
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
                NativeEntitySymbolEntry {
                    ident: `go_right`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
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
                NativeEntitySymbolEntry {
                    ident: `go_left`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
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
                NativeEntitySymbolEntry {
                    ident: `extend_end`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
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
                NativeEntitySymbolEntry {
                    ident: `extend_start`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
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
                NativeEntitySymbolEntry {
                    ident: `find_line_segments`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
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
            ],
        ),
        use_one_trackers: UseExprRules(
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
                    state: UseExprRuleState::Unresolved,
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
                    state: UseExprRuleState::Unresolved,
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
                    state: UseExprRuleState::Unresolved,
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
                    state: UseExprRuleState::Unresolved,
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
                    state: UseExprRuleState::Unresolved,
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
                    state: UseExprRuleState::Unresolved,
                },
            ],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            12,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    11,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 0,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            17,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `convex_component`,
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    16,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 2,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            22,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment`,
                                token_idx: TokenIdx(
                                    20,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    21,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 4,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            28,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `geom2d`,
                                token_idx: TokenIdx(
                                    26,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    27,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 6,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    25,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 7,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `RawContour`,
                        token_idx: TokenIdx(
                            34,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    32,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    33,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 9,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    30,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    31,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 10,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            38,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment`,
                                token_idx: TokenIdx(
                                    36,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    37,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 12,
                            },
                        ),
                    },
                ),
            ],
        },
        errors: [],
    },
)