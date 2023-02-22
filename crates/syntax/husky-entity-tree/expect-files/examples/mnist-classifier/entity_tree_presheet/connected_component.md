Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::connected_component`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "ConnectedComponentDistribution",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 121,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "EffHoles",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 122,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "hole_tmpl",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 123,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "ConnectedComponent",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 124,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "horizontal_extend",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 126,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "find_connected_components",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 127,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 118,
                    use_expr_idx: 3,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::connected_component`,
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
                            2..3,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 119,
                    use_expr_idx: 6,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    9,
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
                    ast_idx: 120,
                    use_expr_idx: 8,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::connected_component`,
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
                            7..8,
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
                                    value: 123,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 121,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                5,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                6,
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
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
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
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 2,
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
                                        value: 120,
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
                            child: 4,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
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
                            child: 5,
                        },
                    ),
                },
                All {
                    star_token: StarToken {
                        token_idx: TokenIdx(
                            17,
                        ),
                    },
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
                            child: 7,
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