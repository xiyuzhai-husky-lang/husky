Ok(
    EntityTreePresheet {
        module_path: `std`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
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
                NativeEntitySymbolEntry {
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
                NativeEntitySymbolEntry {
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
        entity_use_roots: EntityUseExprTrackers(
            [],
        ),
    },
)