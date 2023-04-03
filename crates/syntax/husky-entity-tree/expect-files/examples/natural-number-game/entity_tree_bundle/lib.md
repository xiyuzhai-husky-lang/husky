Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `natural_number_game`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Nat`,
                            visibility: Visibility::PubUnder(
                                `natural_number_game`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
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
                        EntitySymbolEntry {
                            ident: `OddNat`,
                            visibility: Visibility::PubUnder(
                                `natural_number_game`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
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
                        EntitySymbolEntry {
                            ident: `EvenNat`,
                            visibility: Visibility::PubUnder(
                                `natural_number_game`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
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