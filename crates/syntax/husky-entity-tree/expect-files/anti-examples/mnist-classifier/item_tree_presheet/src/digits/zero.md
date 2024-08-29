```rust
EntityTreePresheet {
    module_path: ModulePath(`mnist_classifier::digits::zero`),
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::digits::zero::open_one_match`, `Val`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 25,
                        ident_token: IdentToken {
                            ident: `open_one_match`,
                            token_idx: TokenIdx(
                                6,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                        MajorFormSynNodePath(`mnist_classifier::digits::zero::open_one_match`, `Val`, (0)),
                    ),
                ),
                ident: `open_one_match`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 26,
                        ident_token: IdentToken {
                            ident: `almost_closed`,
                            token_idx: TokenIdx(
                                19,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        1..3,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `almost_closed`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::digits::zero::is_zero`, `Val`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits`),
                        ),
                        ast_idx: 27,
                        ident_token: IdentToken {
                            ident: `is_zero`,
                            token_idx: TokenIdx(
                                50,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        11..24,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::digits::zero::is_zero`, `Val`, (0)),
                    ),
                ),
                ident: `is_zero`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits`),
                ),
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 24,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
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