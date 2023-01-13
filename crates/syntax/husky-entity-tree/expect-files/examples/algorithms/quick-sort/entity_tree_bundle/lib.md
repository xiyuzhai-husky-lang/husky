Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `quick_sort`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `quick_sort`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`quick_sort::quick_sort`, `Function`),
                                    accessibility: Public,
                                    ast_idx: 30,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `quick_sort_aux`,
                            accessibility: PubicUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                    accessibility: PubicUnder(
                                        `quick_sort`,
                                    ),
                                    ast_idx: 31,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `partition`,
                            accessibility: PubicUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`quick_sort::partition`, `Function`),
                                    accessibility: PubicUnder(
                                        `quick_sort`,
                                    ),
                                    ast_idx: 32,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `quick_sort_works_for_integers`,
                            accessibility: PubicUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                                    accessibility: PubicUnder(
                                        `quick_sort`,
                                    ),
                                    ast_idx: 34,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `quick_sort_works_for_strs`,
                            accessibility: PubicUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                                    accessibility: PubicUnder(
                                        `quick_sort`,
                                    ),
                                    ast_idx: 36,
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)