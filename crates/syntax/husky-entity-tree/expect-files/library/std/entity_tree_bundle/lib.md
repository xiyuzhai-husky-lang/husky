Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `std`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: Identifier(
                                "prelude",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 13,
                                    path: `std::prelude`,
                                    accessibility: Accessibility::PublicUnder(
                                        `std`,
                                    ),
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "logic",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 14,
                                    path: `std::logic`,
                                    accessibility: Accessibility::PublicUnder(
                                        `std`,
                                    ),
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "ops",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 15,
                                    path: `std::ops`,
                                    accessibility: Accessibility::PublicUnder(
                                        `std`,
                                    ),
                                    ast_idx: 2,
                                },
                            ),
                        },
                    ],
                ),
                impls: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `std::prelude`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impls: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `std::logic`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impls: [],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 0,
                            use_expr_idx: 2,
                            accessibility: AccessibilityProgress::Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: Identifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 33,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    1..2,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                    ],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [
                    EntityTreeError::UnresolvedIdentifier(
                        IdentifierToken {
                            ident: `core`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `std::ops`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Add",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `std::ops`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 52,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`std::ops::Add`),
                                    ),
                                    accessibility: Accessibility::PublicUnder(
                                        `std::ops`,
                                    ),
                                    ast_idx: 3,
                                },
                            ),
                        },
                    ],
                ),
                impls: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
        ],
        principal_entity_path_expr_arena: Arena {
            data: [],
        },
        impls: [],
    },
)