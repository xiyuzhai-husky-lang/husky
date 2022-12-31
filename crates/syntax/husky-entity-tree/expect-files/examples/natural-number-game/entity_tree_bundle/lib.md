Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `natural_number_game`,
                module_symbols: [
                    ModuleItem {
                        ident: `Nat`,
                        accessibility: PubicUnder(
                            `natural_number_game`,
                        ),
                        ast_idx: 3,
                        path: TypePath(`natural_number_game::Nat`, `Inductive`),
                    },
                    ModuleItem {
                        ident: `OddNat`,
                        accessibility: PubicUnder(
                            `natural_number_game`,
                        ),
                        ast_idx: 9,
                        path: TypePath(`natural_number_game::OddNat`, `Structure`),
                    },
                    ModuleItem {
                        ident: `EvenNat`,
                        accessibility: PubicUnder(
                            `natural_number_game`,
                        ),
                        ast_idx: 10,
                        path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                    },
                ],
            },
        ],
    },
)