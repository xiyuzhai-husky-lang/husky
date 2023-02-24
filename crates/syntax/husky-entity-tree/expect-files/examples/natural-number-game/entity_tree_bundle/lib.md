Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `natural_number_game`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Nat",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 136,
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
                            ident: Identifier(
                                "OddNat",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 137,
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
                            ident: Identifier(
                                "EvenNat",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 138,
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
                impls: [
                    Impl {
                        id: ImplId {
                            module_path: `natural_number_game`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                            },
                            disambiguator: 0,
                        },
                        ast_idx: 6,
                        body: ArenaIdxRange(
                            0..3,
                        ),
                        variant: ImplVariant::Type {
                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                        },
                    },
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
                Root {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 488,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            10,
                        ),
                    },
                    entity_path: ModuleItem(
                        Type(
                            TypePath(
                                Id {
                                    value: 43,
                                },
                            ),
                        ),
                    ),
                },
            ],
        },
        impls: [],
    },
)