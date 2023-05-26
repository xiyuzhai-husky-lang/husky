Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `math`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `algebra`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `math::algebra`,
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `algebra`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `algebraic_geometry`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `math::algebraic_geometry`,
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `algebraic_geometry`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `algebraic_topology`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `math::algebraic_topology`,
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `algebraic_topology`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `category_theory`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `math::category_theory`,
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `category_theory`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `probability`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `math::probability`,
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 4,
                                    ident_token: IdentToken {
                                        ident: `probability`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                },
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `math::algebra`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `math::algebraic_geometry`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `math::algebraic_topology`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `math::category_theory`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `math::probability`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impl_blocks: [],
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
    },
)