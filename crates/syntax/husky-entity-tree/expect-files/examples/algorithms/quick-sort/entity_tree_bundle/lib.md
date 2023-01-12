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
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)