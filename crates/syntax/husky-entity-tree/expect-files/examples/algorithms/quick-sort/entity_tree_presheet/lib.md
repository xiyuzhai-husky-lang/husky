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