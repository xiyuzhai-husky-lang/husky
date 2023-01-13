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
                                    accessibility: PubicUnder(
                                        `std`,
                                    ),
                                    ast_idx: 0,
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
                                    accessibility: PubicUnder(
                                        `std`,
                                    ),
                                    ast_idx: 1,
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
                                    accessibility: PubicUnder(
                                        `std`,
                                    ),
                                    ast_idx: 2,
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
                                    accessibility: PubicUnder(
                                        `std::ops`,
                                    ),
                                    ast_idx: 3,
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)