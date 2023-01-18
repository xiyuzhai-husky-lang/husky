Ok(
    EntityTreePresheet {
        module_path: `quick_sort`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
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
                NativeEntitySymbolEntry {
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
                NativeEntitySymbolEntry {
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
                NativeEntitySymbolEntry {
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
                NativeEntitySymbolEntry {
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
        entity_use_roots: EntityUseExprTrackers(
            [],
        ),
    },
)