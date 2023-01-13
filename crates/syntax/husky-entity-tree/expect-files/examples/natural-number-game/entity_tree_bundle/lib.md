Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `natural_number_game`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Nat`,
                            accessibility: PubicUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                    accessibility: PubicUnder(
                                        `natural_number_game`,
                                    ),
                                    ast_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `OddNat`,
                            accessibility: PubicUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`natural_number_game::OddNat`, `Structure`),
                                    accessibility: PubicUnder(
                                        `natural_number_game`,
                                    ),
                                    ast_idx: 9,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `EvenNat`,
                            accessibility: PubicUnder(
                                `natural_number_game`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                                    accessibility: PubicUnder(
                                        `natural_number_game`,
                                    ),
                                    ast_idx: 10,
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)