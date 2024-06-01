```rust
EntityTreePresheet {
    module_path: `semantics_basics`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            FormSynNodePath(`semantics_basics::some_neural_network`, `Ritchie(
                                Gn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `semantics_basics`,
                        ),
                        ast_idx: 3,
                        ident_token: IdentToken {
                            ident: `some_neural_network`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`semantics_basics::some_neural_network`, `Ritchie(
                                Gn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        0..3,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        FormSynNodePath(`semantics_basics::some_neural_network`, `Ritchie(
                            Gn,
                        )`, (0)),
                    ),
                ),
                ident: `some_neural_network`,
                visibility: Scope::PubUnder(
                    `semantics_basics`,
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
```