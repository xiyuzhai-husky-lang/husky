Ok(
    EntityTreePresheet {
        module_path: `core::basic`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `bool`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::basic::bool`, `Foreign`),
                            accessibility: Public,
                            ast_idx: 0,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Trait`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::basic::Trait`, `Structure`),
                            accessibility: Public,
                            ast_idx: 1,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Module`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::basic::Module`, `Structure`),
                            accessibility: Public,
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