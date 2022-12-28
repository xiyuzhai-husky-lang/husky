Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `std`,
                module_symbols: [
                    Module {
                        ident: `prelude`,
                        accessibility: PubicUnder(
                            `std`,
                        ),
                        ast_idx: 0,
                        module_path: `std::prelude`,
                    },
                    Module {
                        ident: `logic`,
                        accessibility: PubicUnder(
                            `std`,
                        ),
                        ast_idx: 1,
                        module_path: `std::logic`,
                    },
                    Module {
                        ident: `ops`,
                        accessibility: PubicUnder(
                            `std`,
                        ),
                        ast_idx: 2,
                        module_path: `std::ops`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `std::prelude`,
                module_symbols: [],
            },
            EntityTreeSheet {
                module_path: `std::logic`,
                module_symbols: [],
            },
            EntityTreeSheet {
                module_path: `std::ops`,
                module_symbols: [
                    ModuleItem {
                        ident: `Add`,
                        accessibility: PubicUnder(
                            `std::ops`,
                        ),
                        ast_idx: 3,
                        path: TraitPath(`std::ops::Add`),
                    },
                ],
            },
        ],
    },
)