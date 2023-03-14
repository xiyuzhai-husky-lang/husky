Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::nine`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "nine_match",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::nine`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            ast_idx: 44,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "nine_match_refine",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::nine`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            ast_idx: 45,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "is_nine",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::nine`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            ast_idx: 46,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "downmost",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::nine`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            ast_idx: 47,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "big_cc",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::nine`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            ast_idx: 48,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 35,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    1,
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
                    ast_idx: 36,
                    use_expr_idx: 10,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            9..10,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 37,
                    use_expr_idx: 15,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            14..15,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 38,
                    use_expr_idx: 20,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    33,
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
                UseExprRule {
                    ast_idx: 39,
                    use_expr_idx: 23,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    43,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            22..23,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 40,
                    use_expr_idx: 26,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            25..26,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 41,
                    use_expr_idx: 30,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    55,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            29..30,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 42,
                    use_expr_idx: 33,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    63,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            32..33,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 43,
                    use_expr_idx: 36,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::nine`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    69,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            35..36,
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
                        ident: `MnistLabel`,
                        token_idx: TokenIdx(
                            11,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    10,
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
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `cv`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    8,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 1,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `datasets`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    6,
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
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `ml`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    4,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 3,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    2,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `normalize_vmax_f32`,
                        token_idx: TokenIdx(
                            21,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `normalize`,
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    20,
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
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `models`,
                                token_idx: TokenIdx(
                                    17,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    18,
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
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `ml`,
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
                                child: 8,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    14,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `boosting_with_vmax_normalized`,
                        token_idx: TokenIdx(
                            31,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `boosting`,
                                token_idx: TokenIdx(
                                    29,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    30,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 11,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `models`,
                                token_idx: TokenIdx(
                                    27,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    28,
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
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `ml`,
                                token_idx: TokenIdx(
                                    25,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    26,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 13,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    24,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `narrow_down`,
                        token_idx: TokenIdx(
                            41,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `narrow`,
                                token_idx: TokenIdx(
                                    39,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    40,
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
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `models`,
                                token_idx: TokenIdx(
                                    37,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    38,
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
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `ml`,
                                token_idx: TokenIdx(
                                    35,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    36,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 18,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    34,
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
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            47,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `major`,
                                token_idx: TokenIdx(
                                    45,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    46,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 21,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
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
                                child: 22,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            53,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `fermi`,
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
                                child: 24,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    50,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 25,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            61,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    59,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    60,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 27,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `line_segment_sketch`,
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
                                child: 28,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
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
                                child: 29,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_zero`,
                        token_idx: TokenIdx(
                            67,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `zero`,
                                token_idx: TokenIdx(
                                    65,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    66,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 31,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    63,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    64,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 32,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_six`,
                        token_idx: TokenIdx(
                            73,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `six`,
                                token_idx: TokenIdx(
                                    71,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    72,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 34,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    69,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    70,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 35,
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