Ok(
    EntityTreeSheet {
        module_path: `std`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `prelude`,
                    visibility: Visibility::PubUnder(
                        `std`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `std::prelude`,
                            visibility: Visibility::PubUnder(
                                `std`,
                            ),
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `prelude`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `logic`,
                    visibility: Visibility::PubUnder(
                        `std`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `std::logic`,
                            visibility: Visibility::PubUnder(
                                `std`,
                            ),
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `logic`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ops`,
                    visibility: Visibility::PubUnder(
                        `std`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `std::ops`,
                            visibility: Visibility::PubUnder(
                                `std`,
                            ),
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `ops`,
                                token_idx: TokenIdx(
                                    5,
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
)