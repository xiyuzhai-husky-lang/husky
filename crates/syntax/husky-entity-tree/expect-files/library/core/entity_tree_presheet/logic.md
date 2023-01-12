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
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `LogicOr`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`core::logic::LogicOr`, `Inductive`),
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