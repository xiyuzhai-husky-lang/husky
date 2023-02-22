Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "ConcaveComponent",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 74,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "find_concave_components",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 76,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 69,
                    use_expr_idx: 2,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
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
                    ast_idx: 70,
                    use_expr_idx: 6,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
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
                            5..6,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 71,
                    use_expr_idx: 10,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    15,
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
                    ast_idx: 72,
                    use_expr_idx: 13,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    23,
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
                    ast_idx: 73,
                    use_expr_idx: 15,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    29,
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
            ],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [
                All {
                    star_token: StarToken {
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 114,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
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
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 1,
                        },
                    ),
                },
                All {
                    star_token: StarToken {
                        token_idx: TokenIdx(
                            13,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 118,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                11,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                12,
                            ),
                        },
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
                                        value: 114,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                9,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 4,
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
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                8,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 5,
                        },
                    ),
                },
                All {
                    star_token: StarToken {
                        token_idx: TokenIdx(
                            21,
                        ),
                    },
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
                                19,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                20,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 7,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 114,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                17,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                18,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 8,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                15,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 9,
                        },
                    ),
                },
                All {
                    star_token: StarToken {
                        token_idx: TokenIdx(
                            27,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 113,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                25,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                26,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 11,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                23,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                24,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 12,
                        },
                    ),
                },
                All {
                    star_token: StarToken {
                        token_idx: TokenIdx(
                            31,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                29,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 14,
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