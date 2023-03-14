Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "connected_component",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::connected_component`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 12,
                            ident_token: IdentToken {
                                ident: `connected_component`,
                                token_idx: TokenIdx(
                                    8,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "raw_contour",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::raw_contour`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 13,
                            ident_token: IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "geom2d",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::geom2d`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 14,
                            ident_token: IdentToken {
                                ident: `geom2d`,
                                token_idx: TokenIdx(
                                    12,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "line_segment_sketch",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 15,
                            ident_token: IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    14,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "fermi",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::fermi`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 16,
                            ident_token: IdentToken {
                                ident: `fermi`,
                                token_idx: TokenIdx(
                                    16,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "digits",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 17,
                            ident_token: IdentToken {
                                ident: `digits`,
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "major",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::major`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 18,
                            ident_token: IdentToken {
                                ident: `major`,
                                token_idx: TokenIdx(
                                    20,
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
                    ast_idx: 19,
                    use_expr_idx: 1,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    22,
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
                    ast_idx: 20,
                    use_expr_idx: 3,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    26,
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
                    ast_idx: 21,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `major`,
                                token_idx: TokenIdx(
                                    30,
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
                    ast_idx: 22,
                    use_expr_idx: 7,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `digits`,
                                token_idx: TokenIdx(
                                    34,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            6..7,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 23,
                    use_expr_idx: 9,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `fermi`,
                                token_idx: TokenIdx(
                                    38,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            8..9,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 24,
                    use_expr_idx: 11,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    43,
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
                    ast_idx: 25,
                    use_expr_idx: 13,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    47,
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
                UseExprRule {
                    ast_idx: 26,
                    use_expr_idx: 18,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            17..18,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 27,
                    use_expr_idx: 20,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    61,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            19..20,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `BinaryImage28`,
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    23,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `MnistLabel`,
                        token_idx: TokenIdx(
                            28,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
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
                                child: 2,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            32,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `major`,
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
                                child: 4,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            36,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `digits`,
                                token_idx: TokenIdx(
                                    34,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    35,
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
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            40,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `fermi`,
                                token_idx: TokenIdx(
                                    38,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    39,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 8,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `find_raw_contours`,
                        token_idx: TokenIdx(
                            45,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    43,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    44,
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
                            49,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    47,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    48,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `naive_i32`,
                        token_idx: TokenIdx(
                            59,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `naive`,
                                token_idx: TokenIdx(
                                    57,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    58,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 14,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `models`,
                                token_idx: TokenIdx(
                                    55,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    56,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 15,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `ml`,
                                token_idx: TokenIdx(
                                    53,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    54,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 16,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    52,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 17,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            63,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    61,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    62,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 19,
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