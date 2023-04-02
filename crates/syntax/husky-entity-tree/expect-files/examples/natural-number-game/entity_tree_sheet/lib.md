Ok(
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
                            ast_idx: 3,
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
                            ast_idx: 9,
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
                            ast_idx: 10,
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
        impl_blocks: [
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `natural_number_game`,
                        ty_path: TypePath(`natural_number_game::Nat`, `Inductive`),
                        disambiguator: 0,
                    },
                    ast_idx: 6,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            9,
                        ),
                    },
                    ty_expr: 0,
                    body: ArenaIdxRange(
                        0..3,
                    ),
                },
            ),
        ],
        use_expr_rules: UseExprRules(
            [],
        ),
        use_all_rules: UseAllRules(
            [],
        ),
        errors: [],
    },
)