Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `quick_sort`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `quick_sort`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                    ),
                                    visibility: Scope::Pub,
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
                            ident: `quick_sort_aux`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                    ),
                                    visibility: Scope::PubUnder(
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
                            ident: `partition`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`quick_sort::partition`, `Fn`),
                                    ),
                                    visibility: Scope::PubUnder(
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
                            ident: `quick_sort_works_for_integers`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                    ),
                                    visibility: Scope::PubUnder(
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
                            ident: `quick_sort_works_for_strs`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                    ),
                                    visibility: Scope::PubUnder(
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