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
                            path: TypePath(`core::basic::bool`, `Alien`),
                            accessibility: Public,
                            ast_idx: 0,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `never`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::basic::never`, `Alien`),
                            accessibility: Public,
                            ast_idx: 1,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `unit`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::basic::unit`, `Alien`),
                            accessibility: Public,
                            ast_idx: 2,
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
                            ast_idx: 3,
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
                            ast_idx: 4,
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