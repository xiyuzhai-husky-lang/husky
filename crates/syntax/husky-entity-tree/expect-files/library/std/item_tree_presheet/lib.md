EntityTreePresheet {
    module_path: `std`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::Submodule(
                    SubmoduleSynNode {
                        syn_node_path: SubmoduleSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `std`,
                        ),
                        ast_idx: 1,
                        ident_token: IdentToken {
                            ident: `prelude`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::Submodule(
                    Room32,
                    SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                ),
                ident: `prelude`,
                visibility: Scope::PubUnder(
                    `std`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::Submodule(
                    SubmoduleSynNode {
                        syn_node_path: SubmoduleSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `std`,
                        ),
                        ast_idx: 2,
                        ident_token: IdentToken {
                            ident: `logic`,
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::Submodule(
                    Room32,
                    SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                ),
                ident: `logic`,
                visibility: Scope::PubUnder(
                    `std`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::Submodule(
                    SubmoduleSynNode {
                        syn_node_path: SubmoduleSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 3,
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `std`,
                        ),
                        ast_idx: 3,
                        ident_token: IdentToken {
                            ident: `ops`,
                            token_idx: TokenIdx(
                                6,
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::Submodule(
                    Room32,
                    SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                ),
                ident: `ops`,
                visibility: Scope::PubUnder(
                    `std`,
                ),
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