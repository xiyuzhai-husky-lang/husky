Ok(
    EntityTreePresheet {
        module_path: `math`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `algebra`,
                    visibility: Scope::PubUnder(
                        `math`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
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
                NativeEntitySymbolEntry {
                    ident: `algebraic_geometry`,
                    visibility: Scope::PubUnder(
                        `math`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
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
                NativeEntitySymbolEntry {
                    ident: `algebraic_topology`,
                    visibility: Scope::PubUnder(
                        `math`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
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
                NativeEntitySymbolEntry {
                    ident: `category_theory`,
                    visibility: Scope::PubUnder(
                        `math`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
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
                NativeEntitySymbolEntry {
                    ident: `probability`,
                    visibility: Scope::PubUnder(
                        `math`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
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
        use_one_trackers: UseExprRules(
            [],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [],
        },
        errors: [],
    },
)