Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `std`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `prelude`,
                            accessibility: PubicUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `std::prelude`,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `logic`,
                            accessibility: PubicUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `std::logic`,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ops`,
                            accessibility: PubicUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `std::ops`,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `std::prelude`,
                module_specific_symbols: EntitySymbolTable(
                    [],
                ),
            },
            EntityTreeSheet {
                module_path: `std::logic`,
                module_specific_symbols: EntitySymbolTable(
                    [],
                ),
            },
            EntityTreeSheet {
                module_path: `std::ops`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Add`,
                            accessibility: PubicUnder(
                                `std::ops`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`std::ops::Add`),
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)