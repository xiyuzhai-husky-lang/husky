Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::zero`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "open_one_match",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            ast_idx: 33,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "almost_closed",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            ast_idx: 34,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "is_zero",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            ast_idx: 35,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 25,
                    use_expr_idx: 3,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 165,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    1,
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
                    ast_idx: 26,
                    use_expr_idx: 8,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    9,
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
                    ast_idx: 27,
                    use_expr_idx: 13,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    19,
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
                    ast_idx: 28,
                    use_expr_idx: 18,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    29,
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
                    ast_idx: 29,
                    use_expr_idx: 21,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    39,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            20..21,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 30,
                    use_expr_idx: 24,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    45,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            23..24,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 31,
                    use_expr_idx: 28,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            27..28,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 32,
                    use_expr_idx: 31,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    59,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            30..31,
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
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            7,
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
                                            value: 144,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 0,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 166,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 1,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 165,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 2,
                            },
                        ),
                    },
                ),
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 239,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            17,
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
                                            value: 238,
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
                                child: 4,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 151,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 5,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 150,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    12,
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
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                ),
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
                                    value: 241,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            27,
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
                                            value: 240,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 9,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 151,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 10,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 150,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    22,
                                ),
                            ),
                        ),
                        children: Ok(
                            Single {
                                child: 11,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 12,
                            },
                        ),
                    },
                ),
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 222,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            37,
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
                                            value: 221,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 14,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 151,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 15,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 150,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    31,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    32,
                                ),
                            ),
                        ),
                        children: Ok(
                            Single {
                                child: 16,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 17,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            43,
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
                                            value: 142,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    41,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    42,
                                ),
                            ),
                        ),
                        children: Ok(
                            Single {
                                child: 19,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Crate(
                            CrateToken {
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
                            Single {
                                child: 20,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            49,
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
                                            value: 135,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 22,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Crate(
                            CrateToken {
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
                            Single {
                                child: 23,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            57,
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
                            Single {
                                child: 25,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 137,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 26,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Crate(
                            CrateToken {
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
                            Single {
                                child: 27,
                            },
                        ),
                    },
                ),
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 154,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            63,
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
                                            value: 129,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 29,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Crate(
                            CrateToken {
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
                            Single {
                                child: 30,
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