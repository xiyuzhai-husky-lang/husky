Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::two`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "two_match",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 62,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "left_cc_pattern",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 63,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "right_cc_pattern",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 64,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "down_cc_pattern",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 65,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "is_two",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::is_two`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 66,
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
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
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
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
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
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
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
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
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
                    use_expr_idx: 23,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
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
                    ast_idx: 54,
                    use_expr_idx: 26,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
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
                    ast_idx: 55,
                    use_expr_idx: 30,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
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
                    ast_idx: 56,
                    use_expr_idx: 33,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
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
                    ast_idx: 57,
                    use_expr_idx: 36,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
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
                UseExprRule {
                    ast_idx: 58,
                    use_expr_idx: 39,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    75,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            38..39,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 59,
                    use_expr_idx: 42,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    81,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            41..42,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 60,
                    use_expr_idx: 45,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    87,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            44..45,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 61,
                    use_expr_idx: 48,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    93,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            47..48,
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                Parent {
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
                All {
                    star_token: StarToken(
                        TokenIdx(
                            47,
                        ),
                    ),
                },
                Parent {
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
                Parent {
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
                All {
                    star_token: StarToken(
                        TokenIdx(
                            53,
                        ),
                    ),
                },
                Parent {
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
                Parent {
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
                All {
                    star_token: StarToken(
                        TokenIdx(
                            61,
                        ),
                    ),
                },
                Parent {
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
                Parent {
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
                Parent {
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
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 156,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            67,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 134,
                                    },
                                ),
                            ),
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
                        Single {
                            child: 31,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
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
                        Single {
                            child: 32,
                        },
                    ),
                },
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
                            73,
                        ),
                    },
                },
                Parent {
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
                        Single {
                            child: 34,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
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
                        Single {
                            child: 35,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 159,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            79,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 132,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                77,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                78,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 37,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                75,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                76,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 38,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            85,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 130,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                83,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                84,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 40,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                81,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                82,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 41,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 160,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            91,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 128,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                89,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                90,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 43,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                87,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                88,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 44,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 155,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            97,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 131,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                95,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                96,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 46,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                93,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                94,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 47,
                        },
                    ),
                },
            ],
        },
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)