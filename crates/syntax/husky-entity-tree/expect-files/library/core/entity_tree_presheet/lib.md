Ok(
    EntityTreePresheet {
        module_path: `core`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `basic`,
                    accessibility: PubicUnder(
                        `core`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `core::basic`,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `logic`,
                    accessibility: PubicUnder(
                        `core`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `core::logic`,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `num`,
                    accessibility: PubicUnder(
                        `core`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `core::num`,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `ops`,
                    accessibility: PubicUnder(
                        `core`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `core::ops`,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `prelude`,
                    accessibility: PubicUnder(
                        `core`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `core::prelude`,
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