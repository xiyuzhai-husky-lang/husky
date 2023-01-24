Ok(
    EntityTreePresheet {
        module_path: `core::marker`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `Copy`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TraitPath(`core::marker::Copy`),
                            accessibility: Public,
                            ast_idx: 0,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Sized`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TraitPath(`core::marker::Sized`),
                            accessibility: Public,
                            ast_idx: 1,
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