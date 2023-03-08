Ok(
    EntityTreePresheet {
        module_path: `std::ops`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "Add",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `std::ops`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Trait(
                                TraitPath(`std::ops::Add`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `std::ops`,
                            ),
                            ast_idx: 3,
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