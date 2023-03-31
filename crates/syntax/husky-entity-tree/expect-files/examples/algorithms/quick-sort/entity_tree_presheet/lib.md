Ok(
    EntityTreePresheet {
        module_path: `quick_sort`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `quick_sort`,
                    visibility: Visibility::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::quick_sort`, `Fn`),
                            ),
                            visibility: Visibility::Pub,
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
                    ident: `quick_sort_aux`,
                    visibility: Visibility::PubUnder(
                        `quick_sort`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::quick_sort_aux`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
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
                    ident: `partition`,
                    visibility: Visibility::PubUnder(
                        `quick_sort`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::partition`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
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
                    ident: `quick_sort_works_for_integers`,
                    visibility: Visibility::PubUnder(
                        `quick_sort`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
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
                    ident: `quick_sort_works_for_strs`,
                    visibility: Visibility::PubUnder(
                        `quick_sort`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
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
        errors: [],
    },
)