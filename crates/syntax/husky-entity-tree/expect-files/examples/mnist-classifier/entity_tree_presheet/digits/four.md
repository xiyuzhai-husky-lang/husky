Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::four`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "left_components",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 44,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "left_coordinate_max",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 45,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "components_max_downwards",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 46,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "components_max_heights",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 47,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "is_four",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 48,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "displacement_downwards",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 49,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "cc_box_heights",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 50,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 37,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::four`,
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
                    ast_idx: 38,
                    use_expr_idx: 10,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::four`,
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
                    ast_idx: 39,
                    use_expr_idx: 15,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::four`,
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
                    ast_idx: 40,
                    use_expr_idx: 20,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::four`,
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
                    ast_idx: 41,
                    use_expr_idx: 23,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::four`,
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
                    ast_idx: 42,
                    use_expr_idx: 26,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::four`,
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
                    ast_idx: 43,
                    use_expr_idx: 30,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::four`,
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
                            11,
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
                                            value: 165,
                                        },
                                    ),
                                ),
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
                                            value: 166,
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
                                child: 2,
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
                                child: 3,
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
                                child: 4,
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
                            21,
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
                                            value: 151,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 7,
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
                                child: 8,
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
                                child: 9,
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
                            31,
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
                                            value: 151,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 12,
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
                                child: 13,
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
                                child: 14,
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
                            41,
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
                                            value: 151,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 17,
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
                                child: 18,
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
                                child: 19,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            47,
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
                                child: 21,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Crate(
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
                            Single {
                                child: 22,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            53,
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
                                child: 24,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Crate(
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
                            Single {
                                child: 25,
                            },
                        ),
                    },
                ),
                All {
                    star_token: StarToken(
                        TokenIdx(
                            61,
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
                                child: 27,
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
                            Single {
                                child: 28,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Crate(
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
                            Single {
                                child: 29,
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