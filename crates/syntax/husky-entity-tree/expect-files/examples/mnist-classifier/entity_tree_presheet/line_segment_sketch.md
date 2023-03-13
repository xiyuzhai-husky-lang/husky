Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "concave_component",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::concave_component`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 159,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "convex_component",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::convex_component`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 160,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "convexity",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::convexity`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 161,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "line_segment",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::line_segment`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 162,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "LineSegmentStroke",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 169,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "LineSegmentSketch",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 171,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "go_right",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 173,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "go_left",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 174,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "extend_end",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 175,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "extend_start",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 176,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "find_line_segments",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 177,
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
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::Public,
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 138,
                                        },
                                    ),
                                ),
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
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::Public,
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 139,
                                        },
                                    ),
                                ),
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
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::Public,
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 141,
                                        },
                                    ),
                                ),
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
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
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
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
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
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 141,
                                        },
                                    ),
                                ),
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
                All {
                    star_token: StarToken(
                        TokenIdx(
                            12,
                        ),
                    ),
                },
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 138,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 0,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            17,
                        ),
                    ),
                },
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 139,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 2,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            22,
                        ),
                    ),
                },
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 141,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 4,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            28,
                        ),
                    ),
                },
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 136,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 6,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Crate(
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
                            Single {
                                child: 7,
                            },
                        ),
                    },
                ),
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 174,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            34,
                        ),
                    },
                },
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 143,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 9,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Crate(
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
                            Single {
                                child: 10,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            38,
                        ),
                    ),
                },
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 141,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 12,
                            },
                        ),
                    },
                ),
            ],
        },
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)