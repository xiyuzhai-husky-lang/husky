Ok(
    EntityTreePresheet {
        module_path: `core::cmp`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `PartialEq`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TraitPath(`core::cmp::PartialEq`),
                            accessibility: Public,
                            ast_idx: 0,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Eq`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TraitPath(`core::cmp::Eq`),
                            accessibility: Public,
                            ast_idx: 1,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `PartialOrd`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TraitPath(`core::cmp::PartialOrd`),
                            accessibility: Public,
                            ast_idx: 2,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Ord`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TraitPath(`core::cmp::Ord`),
                            accessibility: Public,
                            ast_idx: 3,
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