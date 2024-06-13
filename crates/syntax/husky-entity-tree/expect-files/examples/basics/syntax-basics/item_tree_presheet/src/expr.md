```rust
EntityTreePresheet {
    module_path: ModulePath(`syntax_basics::expr`),
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`syntax_basics::expr::nested`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::expr`),
                        ),
                        ast_idx: 3,
                        ident_token: IdentToken {
                            ident: `nested`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`syntax_basics::expr::nested`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`syntax_basics::expr::nested`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `nested`,
                visibility: Scope::PubUnder(
                    ModulePath(`syntax_basics::expr`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`syntax_basics::expr::closure_inline`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::expr`),
                        ),
                        ast_idx: 4,
                        ident_token: IdentToken {
                            ident: `closure_inline`,
                            token_idx: TokenIdx(
                                13,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`syntax_basics::expr::closure_inline`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`syntax_basics::expr::closure_inline`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `closure_inline`,
                visibility: Scope::PubUnder(
                    ModulePath(`syntax_basics::expr`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`syntax_basics::expr::closure_nested`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::expr`),
                        ),
                        ast_idx: 5,
                        ident_token: IdentToken {
                            ident: `closure_nested`,
                            token_idx: TokenIdx(
                                29,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`syntax_basics::expr::closure_nested`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        2..3,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`syntax_basics::expr::closure_nested`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `closure_nested`,
                visibility: Scope::PubUnder(
                    ModulePath(`syntax_basics::expr`),
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