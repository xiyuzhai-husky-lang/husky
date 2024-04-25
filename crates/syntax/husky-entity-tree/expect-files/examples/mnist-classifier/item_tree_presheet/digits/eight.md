```rust
EntityTreePresheet {
    module_path: `mnist_classifier::digits::eight`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            FormSynNodePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::digits::eight`,
                        ),
                        ast_idx: 14,
                        ident_token: IdentToken {
                            ident: `upper_mouth_match`,
                            token_idx: TokenIdx(
                                6,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                        FormSynNodePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`, (0)),
                    ),
                ),
                ident: `upper_mouth_match`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::digits::eight`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            FormSynNodePath(`mnist_classifier::digits::eight::is_eight`, `Val`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 15,
                        ident_token: IdentToken {
                            ident: `is_eight`,
                            token_idx: TokenIdx(
                                23,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        6..9,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        FormSynNodePath(`mnist_classifier::digits::eight::is_eight`, `Val`, (0)),
                    ),
                ),
                ident: `is_eight`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::digits`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            FormSynNodePath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::digits::eight`,
                        ),
                        ast_idx: 16,
                        ident_token: IdentToken {
                            ident: `big_mouth`,
                            token_idx: TokenIdx(
                                73,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        11..13,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        FormSynNodePath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `big_mouth`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::digits::eight`,
                ),
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 13,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    `mnist_classifier::digits::eight`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Super(
                        SuperToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        0..1,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    use_expr_arena: Arena {
        data: [
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        4,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::Super(
                        SuperToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                3,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 0,
                        },
                    ),
                },
            ),
        ],
    },
}
```