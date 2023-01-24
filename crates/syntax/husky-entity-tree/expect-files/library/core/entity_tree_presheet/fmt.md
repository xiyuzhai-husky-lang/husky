Ok(
    EntityTreePresheet {
        module_path: `core::fmt`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `Debug`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TraitPath(`core::fmt::Debug`),
                            accessibility: Public,
                            ast_idx: 0,
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