Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `quick_sort`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntitySynNode::ModuleItem(
                                ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    0..2,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            ),
                            node_path: EntitySynNodePath::ModuleItem(
                                ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
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
                            node: EntitySynNode::ModuleItem(
                                ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    6..7,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            ),
                            node_path: EntitySynNodePath::ModuleItem(
                                ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
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
                            node: EntitySynNode::ModuleItem(
                                ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    18..24,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            ),
                            node_path: EntitySynNodePath::ModuleItem(
                                ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
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
                            node: EntitySynNode::ModuleItem(
                                ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    24..27,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            ),
                            node_path: EntitySynNodePath::ModuleItem(
                                ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
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
                            node: EntitySynNode::ModuleItem(
                                ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    27..30,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            ),
                            node_path: EntitySynNodePath::ModuleItem(
                                ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
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
                                node: ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    0..2,
                                                ),
                                            },
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
                                node: ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    6..7,
                                                ),
                                            },
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
                                node: ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    18..24,
                                                ),
                                            },
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
                                node: ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    24..27,
                                                ),
                                            },
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
                                node: ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
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
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    27..30,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                        },
                    ],
                ),
                impl_block_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
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