```rust
EntityTreePresheet {
    module_path: ModulePath(`mnist_classifier::digits::five`),
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::digits::five::is_five`, `Val`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits`),
                        ),
                        ast_idx: 2,
                        ident_token: IdentToken {
                            ident: `is_five`,
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
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
                        MajorFormSynNodePath(`mnist_classifier::digits::five::is_five`, `Val`, (0)),
                    ),
                ),
                ident: `is_five`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits`),
                ),
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 1,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::five`),
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