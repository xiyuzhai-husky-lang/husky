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
                            symbol: EntitySymbol::ModuleItem(
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
                        EntitySymbolEntry {
                            ident: Ident(
                                "quick_sort_aux",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
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
                        EntitySymbolEntry {
                            ident: Ident(
                                "partition",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
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
                        EntitySymbolEntry {
                            ident: Ident(
                                "quick_sort_works_for_integers",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
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
                        EntitySymbolEntry {
                            ident: Ident(
                                "quick_sort_works_for_strs",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
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