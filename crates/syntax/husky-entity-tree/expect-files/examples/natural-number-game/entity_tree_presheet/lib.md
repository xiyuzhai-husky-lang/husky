Ok(
    EntityTreePresheet {
        module_path: `natural_number_game`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
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
                NativeEntitySymbolEntry {
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
                NativeEntitySymbolEntry {
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
        entity_use_roots: EntityUseExprTrackers(
            [],
        ),
    },
)