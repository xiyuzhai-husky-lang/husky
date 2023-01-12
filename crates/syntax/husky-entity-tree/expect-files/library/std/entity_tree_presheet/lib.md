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