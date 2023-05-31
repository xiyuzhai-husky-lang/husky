Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `natural_number_game`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Nat`,
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`natural_number_game::Nat`, `Inductive`),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `natural_number_game`,
                                    ),
                                    ast_idx: 5,
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
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`natural_number_game::OddNat`, `Structure`),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `natural_number_game`,
                                    ),
                                    ast_idx: 9,
                                    ident_token: IdentToken {
                                        ident: `OddNat`,
                                        token_idx: TokenIdx(
                                            83,
                                        ),
                                    },
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `EvenNat`,
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`natural_number_game::EvenNat`, `Structure`),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `natural_number_game`,
                                    ),
                                    ast_idx: 10,
                                    ident_token: IdentToken {
                                        ident: `EvenNat`,
                                        token_idx: TokenIdx(
                                            112,
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
                            body: Type(
                                TypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        2..5,
                                    ),
                                },
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
        ],
        principal_entity_path_expr_arena: Arena {
            data: [
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Nat`,
                        token_idx: TokenIdx(
                            10,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`natural_number_game::Nat`, `Inductive`),
                        ),
                    ),
                },
            ],
        },
    },
)