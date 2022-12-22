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
                        module_path: `std::prelude`,
                    },
                    Module {
                        ident: `logic`,
                        accessibility: PubicUnder(
                            `std`,
                        ),
                        module_path: `std::logic`,
                    },
                    Module {
                        ident: `ops`,
                        accessibility: PubicUnder(
                            `std`,
                        ),
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
                        ident: `Output`,
                        accessibility: PubicUnder(
                            `std::ops`,
                        ),
                        ast_idx: 0,
                        path: `std::ops::Output`,
                    },
                    ModuleItem {
                        ident: `Add`,
                        accessibility: PubicUnder(
                            `std::ops`,
                        ),
                        ast_idx: 3,
                        path: `std::ops::Add`,
                    },
                ],
            },
        ],
    },
)