Ok(
    EntityTreePresheet {
        module_path: `std`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "prelude",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `std`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `std::prelude`,
                            accessibility: Accessibility::PublicUnder(
                                `std`,
                            ),
                            ast_idx: 0,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "logic",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `std`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `std::logic`,
                            accessibility: Accessibility::PublicUnder(
                                `std`,
                            ),
                            ast_idx: 1,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "ops",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `std`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `std::ops`,
                            accessibility: Accessibility::PublicUnder(
                                `std`,
                            ),
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