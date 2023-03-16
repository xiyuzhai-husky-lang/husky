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
                                FormPath(`quick_sort::quick_sort`, `Fn`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 30,
                            ident_token: IdentToken {
                                ident: `quick_sort`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
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
                                FormPath(`quick_sort::quick_sort_aux`, `Fn`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            ast_idx: 31,
                            ident_token: IdentToken {
                                ident: `quick_sort_aux`,
                                token_idx: TokenIdx(
                                    41,
                                ),
                            },
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
                                FormPath(`quick_sort::partition`, `Fn`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            ast_idx: 32,
                            ident_token: IdentToken {
                                ident: `partition`,
                                token_idx: TokenIdx(
                                    102,
                                ),
                            },
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
                                FormPath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            ast_idx: 34,
                            ident_token: IdentToken {
                                ident: `quick_sort_works_for_integers`,
                                token_idx: TokenIdx(
                                    227,
                                ),
                            },
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
                                FormPath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            ast_idx: 36,
                            ident_token: IdentToken {
                                ident: `quick_sort_works_for_strs`,
                                token_idx: TokenIdx(
                                    287,
                                ),
                            },
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