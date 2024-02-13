EntityTreeSheet {
    module_path: `syntax_basics::expr`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 10,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `syntax_basics::expr`,
                        ),
                        ast_idx: 2,
                        ident_token: IdentToken {
                            ident: `nested`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`syntax_basics::expr::nested`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 10,
                                },
                            ),
                        ),
                    ),
                ),
                ident: `nested`,
                visibility: Scope::PubUnder(
                    `syntax_basics::expr`,
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `nested`,
                visibility: Scope::PubUnder(
                    `syntax_basics::expr`,
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Fugitive(
                        FugitivePath(`syntax_basics::expr::nested`, `Ritchie(
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