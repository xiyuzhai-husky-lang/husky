Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `quick_sort`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: Ident(
                                "quick_sort",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 53,
                                    path: ModuleItemPath::Form(
                                        FormPath(`quick_sort::quick_sort`, `Function`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 30,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Ident(
                                "quick_sort_aux",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 54,
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
                        EntitySymbolEntry {
                            ident: Ident(
                                "partition",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 55,
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
                        EntitySymbolEntry {
                            ident: Ident(
                                "quick_sort_works_for_integers",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 56,
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
                        EntitySymbolEntry {
                            ident: Ident(
                                "quick_sort_works_for_strs",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 57,
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
                impls: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
        ],
        principal_entity_path_expr_arena: Arena {
            data: [],
        },
        impls: [],
    },
)