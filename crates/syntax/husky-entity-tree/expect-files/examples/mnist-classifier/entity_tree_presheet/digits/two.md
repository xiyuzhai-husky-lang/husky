Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::two`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `two_match`,
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                            ),
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 54,
                            ident_token: IdentToken {
                                ident: `two_match`,
                                token_idx: TokenIdx(
                                    47,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `left_cc_pattern`,
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                            ),
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 55,
                            ident_token: IdentToken {
                                ident: `left_cc_pattern`,
                                token_idx: TokenIdx(
                                    64,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `right_cc_pattern`,
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                            ),
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 56,
                            ident_token: IdentToken {
                                ident: `right_cc_pattern`,
                                token_idx: TokenIdx(
                                    93,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `down_cc_pattern`,
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                            ),
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 57,
                            ident_token: IdentToken {
                                ident: `down_cc_pattern`,
                                token_idx: TokenIdx(
                                    122,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_two`,
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::is_two`, `Feature`),
                            ),
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 58,
                            ident_token: IdentToken {
                                ident: `is_two`,
                                token_idx: TokenIdx(
                                    151,
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
                    ast_idx: 49,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Visibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
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
                    ast_idx: 50,
                    use_expr_idx: 10,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Visibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
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
                    ast_idx: 51,
                    use_expr_idx: 15,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Visibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
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
                    ast_idx: 52,
                    use_expr_idx: 20,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Visibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
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
                    ast_idx: 53,
                    use_expr_idx: 22,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Visibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    43,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            21..22,
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                        parent_name_token: NameToken::Ident(
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
                            45,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Crate(
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
                                child: 21,
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