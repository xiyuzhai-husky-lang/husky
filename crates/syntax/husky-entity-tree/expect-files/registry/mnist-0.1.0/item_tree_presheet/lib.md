EntityTreePresheet {
    module_path: `mnist`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 17,
                        ident_token: IdentToken {
                            ident: `MnistLabel`,
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist::MnistLabel`, `Enum`),
                            variants: Some(
                                TypeVariants {
                                    ast_idx_range: ArenaIdxRange(
                                        1..11,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                ident: `MnistLabel`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 19,
                        ident_token: IdentToken {
                            ident: `BinaryImage28`,
                            token_idx: TokenIdx(
                                33,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist::BinaryImage28`, `Extern`),
                            variants: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                    ),
                ),
                ident: `BinaryImage28`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 23,
                        ident_token: IdentToken {
                            ident: `BinaryGrid28`,
                            token_idx: TokenIdx(
                                74,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                            variants: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 3,
                                },
                            ),
                        ),
                    ),
                ),
                ident: `BinaryGrid28`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 27,
                        ident_token: IdentToken {
                            ident: `input`,
                            token_idx: TokenIdx(
                                115,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist::input`, `Val`),
                            body: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 4,
                                },
                            ),
                        ),
                    ),
                ),
                ident: `input`,
                visibility: Scope::Pub,
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    use_expr_arena: Arena {
        data: [],
    },
}