Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `quick_sort`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            node_path: EntityNodePath::ModuleItem(
                                ModuleItemNodePath::Fugitive(
                                    FugitiveNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                            ident: `quick_sort`,
                            visibility: Scope::Pub,
                        },
                        EntityNodeEntry {
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            node_path: EntityNodePath::ModuleItem(
                                ModuleItemNodePath::Fugitive(
                                    FugitiveNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                            ident: `quick_sort_aux`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                        },
                        EntityNodeEntry {
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::partition`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            node_path: EntityNodePath::ModuleItem(
                                ModuleItemNodePath::Fugitive(
                                    FugitiveNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`quick_sort::partition`, `Fn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                            ident: `partition`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                        },
                        EntityNodeEntry {
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            node_path: EntityNodePath::ModuleItem(
                                ModuleItemNodePath::Fugitive(
                                    FugitiveNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                            ident: `quick_sort_works_for_integers`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                        },
                        EntityNodeEntry {
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            node_path: EntityNodePath::ModuleItem(
                                ModuleItemNodePath::Fugitive(
                                    FugitiveNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                            ident: `quick_sort_works_for_strs`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                        },
                    ],
                },
                entity_symbol_table: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `quick_sort`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            },
                        },
                        EntitySymbolEntry {
                            ident: `quick_sort_aux`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            },
                        },
                        EntitySymbolEntry {
                            ident: `partition`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`quick_sort::partition`, `Fn`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::partition`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            },
                        },
                        EntitySymbolEntry {
                            ident: `quick_sort_works_for_integers`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            },
                        },
                        EntitySymbolEntry {
                            ident: `quick_sort_works_for_strs`,
                            visibility: Scope::PubUnder(
                                `quick_sort`,
                            ),
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
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
                            },
                        },
                    ],
                ),
                impl_block_node_table: [],
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