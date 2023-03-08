Ok(
    EntityTreePresheet {
        module_path: `core::marker`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "Copy",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 0,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "Sized",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Trait(
                                TraitPath(`core::marker::Sized`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 1,
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