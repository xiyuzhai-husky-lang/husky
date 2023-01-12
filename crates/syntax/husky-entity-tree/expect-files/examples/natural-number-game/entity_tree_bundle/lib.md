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
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)