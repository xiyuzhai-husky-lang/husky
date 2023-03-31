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
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 203,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `OddNat`,
                            visibility: Visibility::PubUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 204,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `EvenNat`,
                            visibility: Visibility::PubUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 205,
                                    },
                                ),
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