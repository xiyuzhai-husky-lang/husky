Ok(
    EntityTreePresheet {
        module_path: `natural_number_game`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `Nat`,
                    visibility: Visibility::PubUnder(
                        `natural_number_game`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`natural_number_game::Nat`, `Inductive`),
                            ),
                            visibility: Visibility::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Nat`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `OddNat`,
                    visibility: Visibility::PubUnder(
                        `natural_number_game`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`natural_number_game::OddNat`, `Structure`),
                            ),
                            visibility: Visibility::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 8,
                            ident_token: IdentToken {
                                ident: `OddNat`,
                                token_idx: TokenIdx(
                                    85,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `EvenNat`,
                    visibility: Visibility::PubUnder(
                        `natural_number_game`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`natural_number_game::EvenNat`, `Structure`),
                            ),
                            visibility: Visibility::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 9,
                            ident_token: IdentToken {
                                ident: `EvenNat`,
                                token_idx: TokenIdx(
                                    114,
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