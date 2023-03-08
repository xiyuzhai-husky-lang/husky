Ok(
    EntityTreePresheet {
        module_path: `quick_sort`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "quick_sort",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::quick_sort`, `Function`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 30,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "quick_sort_aux",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `quick_sort`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::quick_sort_aux`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            ast_idx: 31,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "partition",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `quick_sort`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::partition`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            ast_idx: 32,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "quick_sort_works_for_integers",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `quick_sort`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            ast_idx: 34,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "quick_sort_works_for_strs",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `quick_sort`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            ast_idx: 36,
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