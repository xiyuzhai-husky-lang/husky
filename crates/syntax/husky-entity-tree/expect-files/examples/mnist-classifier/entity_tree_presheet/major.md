Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::major`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "connected_components",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 23,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "major_connected_component",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 24,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "ignored_connected_components_row_span_sum_sum",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 25,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "major_raw_contours",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 26,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "major_raw_contour",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 27,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "major_line_segment_sketch",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 28,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "major_concave_components",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 29,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 18,
                    use_expr_idx: 2,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::major`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::major`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    7,
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
                    ast_idx: 20,
                    use_expr_idx: 8,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::major`,
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
                    ast_idx: 21,
                    use_expr_idx: 12,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::major`,
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
                            11..12,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 22,
                    use_expr_idx: 14,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::major`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    27,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            13..14,
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
                            5,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 112,
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
                            child: 0,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
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
                            child: 1,
                        },
                    ),
                },
                All {
                    star_token: StarToken(
                        TokenIdx(
                            11,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 132,
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
                            child: 3,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
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
                            child: 4,
                        },
                    ),
                },
                All {
                    star_token: StarToken(
                        TokenIdx(
                            17,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 126,
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
                All {
                    star_token: StarToken(
                        TokenIdx(
                            25,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 127,
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
                            child: 9,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 126,
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
                            child: 10,
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
                            child: 11,
                        },
                    ),
                },
                All {
                    star_token: StarToken(
                        TokenIdx(
                            29,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
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
                            child: 13,
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