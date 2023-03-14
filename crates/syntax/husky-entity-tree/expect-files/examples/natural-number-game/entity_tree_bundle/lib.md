Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `natural_number_game`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: Ident(
                                "Nat",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 142,
                                    path: ModuleItemPath::Type(
                                        TypePath(`natural_number_game::Nat`, `Inductive`),
                                    ),
                                    accessibility: Accessibility::PublicUnder(
                                        `natural_number_game`,
                                    ),
                                    ast_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Ident(
                                "OddNat",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 143,
                                    path: ModuleItemPath::Type(
                                        TypePath(`natural_number_game::OddNat`, `Structure`),
                                    ),
                                    accessibility: Accessibility::PublicUnder(
                                        `natural_number_game`,
                                    ),
                                    ast_idx: 9,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Ident(
                                "EvenNat",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 144,
                                    path: ModuleItemPath::Type(
                                        TypePath(`natural_number_game::EvenNat`, `Structure`),
                                    ),
                                    accessibility: Accessibility::PublicUnder(
                                        `natural_number_game`,
                                    ),
                                    ast_idx: 10,
                                },
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module: `natural_number_game`,
                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                                disambiguator: 0,
                            },
                            ast_idx: 6,
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
        impl_blocks: ImplBlockBundle {
            all_ty_impl_blocks: [],
            all_ty_as_trai_impl_blocks: [],
            all_ill_formed_impl_blocks: [],
        },
    },
)