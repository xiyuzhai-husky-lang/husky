Ok(
    EntityTreePresheet {
        module_path: `core::ops`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `Add`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TraitPath(`core::ops::Add`),
                            accessibility: Public,
                            ast_idx: 5,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Sub`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TraitPath(`core::ops::Sub`),
                            accessibility: Public,
                            ast_idx: 7,
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