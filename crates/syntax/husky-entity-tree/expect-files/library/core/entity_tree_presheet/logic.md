Ok(
    EntityTreePresheet {
        module_path: `core::logic`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `LogicAnd`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::logic::LogicAnd`, `Structure`),
                            accessibility: Public,
                            ast_idx: 0,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `LogicOr`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::logic::LogicOr`, `Inductive`),
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