Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::seven`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "simple_seven_match",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::seven`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            ast_idx: 47,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "simple_leftdown_pattern",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::seven`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            ast_idx: 48,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "special_seven_match",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::seven`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            ast_idx: 49,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "leftupcc_pattern",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::seven`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            ast_idx: 50,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "leftdowncc_pattern",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::seven`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            ast_idx: 51,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "is_seven",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits::seven`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            ast_idx: 52,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 39,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 142,
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
                    ast_idx: 40,
                    use_expr_idx: 8,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    13,
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
                    ast_idx: 41,
                    use_expr_idx: 11,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    19,
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
                    ast_idx: 42,
                    use_expr_idx: 16,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 142,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    25,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            15..16,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 43,
                    use_expr_idx: 19,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    35,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            18..19,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 44,
                    use_expr_idx: 22,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    41,
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
                UseExprRule {
                    ast_idx: 45,
                    use_expr_idx: 26,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::seven`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    47,
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
                    ast_idx: 46,
                    use_expr_idx: 29,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits::seven`,
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
                            28..29,
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
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 140,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            11,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 137,
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
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 158,
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
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 159,
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
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 143,
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
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 142,
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
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 212,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            17,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 135,
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
                            child: 6,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
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
                            child: 7,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 213,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            23,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 135,
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
                            child: 9,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
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
                            child: 10,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            33,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 214,
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
                            child: 12,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 144,
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
                            child: 13,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 143,
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
                            child: 14,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 142,
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
                            child: 15,
                        },
                    ),
                },
                All {
                    star_token: StarToken(
                        TokenIdx(
                            39,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 135,
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
                    parent_name_token: Crate(
                        CrateToken {
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
                All {
                    star_token: StarToken(
                        TokenIdx(
                            45,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 128,
                                    },
                                ),
                            ),
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
                            child: 20,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
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
                            child: 21,
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
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 131,
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
                            child: 23,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 130,
                                    },
                                ),
                            ),
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
                            child: 24,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
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
                            child: 25,
                        },
                    ),
                },
                All {
                    star_token: StarToken(
                        TokenIdx(
                            59,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 117,
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
                            child: 27,
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
                            child: 28,
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