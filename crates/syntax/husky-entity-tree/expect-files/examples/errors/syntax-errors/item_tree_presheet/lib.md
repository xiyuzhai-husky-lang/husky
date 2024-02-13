EntityTreePresheet {
    module_path: `syntax_errors`,
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
                            `syntax_errors`,
                        ),
                        ast_idx: 1,
                        ident_token: IdentToken {
                            ident: `ast`,
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
                ident: `ast`,
                visibility: Scope::PubUnder(
                    `syntax_errors`,
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
                            `syntax_errors`,
                        ),
                        ast_idx: 2,
                        ident_token: IdentToken {
                            ident: `uses`,
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
                ident: `uses`,
                visibility: Scope::PubUnder(
                    `syntax_errors`,
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