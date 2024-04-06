```rust
EntityTreeSheet {
    module_path: `quick_sort`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(`quick_sort::quick_sort`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 26,
                        ident_token: IdentToken {
                            ident: `quick_sort`,
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`quick_sort::quick_sort`, `Ritchie(
                                Fn,
                            )`),
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
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(`quick_sort::quick_sort`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `quick_sort`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `quick_sort`,
                        ),
                        ast_idx: 27,
                        ident_token: IdentToken {
                            ident: `quick_sort_aux`,
                            token_idx: TokenIdx(
                                43,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                Fn,
                            )`),
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
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `quick_sort_aux`,
                visibility: Scope::PubUnder(
                    `quick_sort`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(`quick_sort::partition`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `quick_sort`,
                        ),
                        ast_idx: 28,
                        ident_token: IdentToken {
                            ident: `partition`,
                            token_idx: TokenIdx(
                                105,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`quick_sort::partition`, `Ritchie(
                                Fn,
                            )`),
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
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(`quick_sort::partition`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `partition`,
                visibility: Scope::PubUnder(
                    `quick_sort`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `quick_sort`,
                        ),
                        ast_idx: 30,
                        ident_token: IdentToken {
                            ident: `quick_sort_works_for_integers`,
                            token_idx: TokenIdx(
                                231,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        24..25,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `quick_sort_works_for_integers`,
                visibility: Scope::PubUnder(
                    `quick_sort`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `quick_sort`,
                        ),
                        ast_idx: 32,
                        ident_token: IdentToken {
                            ident: `quick_sort_works_for_strs`,
                            token_idx: TokenIdx(
                                268,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        25..26,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `quick_sort_works_for_strs`,
                visibility: Scope::PubUnder(
                    `quick_sort`,
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `quick_sort`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort`, `Ritchie(
                            Fn,
                        )`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `quick_sort_aux`,
                visibility: Scope::PubUnder(
                    `quick_sort`,
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                            Fn,
                        )`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `partition`,
                visibility: Scope::PubUnder(
                    `quick_sort`,
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::partition`, `Ritchie(
                            Fn,
                        )`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `quick_sort_works_for_integers`,
                visibility: Scope::PubUnder(
                    `quick_sort`,
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                            Fn,
                        )`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `quick_sort_works_for_strs`,
                visibility: Scope::PubUnder(
                    `quick_sort`,
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                            Fn,
                        )`),
                    ),
                },
            },
        ],
    ),
    impl_block_syn_node_table: [],
    once_use_rules: OnceUseRules(
        [],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    errors: [],
}
```