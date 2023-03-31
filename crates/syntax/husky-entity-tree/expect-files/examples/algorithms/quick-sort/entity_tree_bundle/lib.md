Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `quick_sort`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `quick_sort`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 108,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `quick_sort_aux`,
                            visibility: Visibility::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 109,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `partition`,
                            visibility: Visibility::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 110,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `quick_sort_works_for_integers`,
                            visibility: Visibility::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 111,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `quick_sort_works_for_strs`,
                            visibility: Visibility::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 112,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
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
    },
)