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
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Trait`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::basic::Trait`, `Structure`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Module`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::basic::Module`, `Structure`),
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