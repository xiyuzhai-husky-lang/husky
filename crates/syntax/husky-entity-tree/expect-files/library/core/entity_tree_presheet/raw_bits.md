Ok(
    EntityTreePresheet {
        module_path: `core::raw_bits`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "r32",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::raw_bits::r32`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 2,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [],
        },
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)