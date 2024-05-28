```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: `mnist`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    FormSynNodePath(`mnist::Task`, `TypeAlias`, (0)),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist`,
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `Task`,
                                    token_idx: TokenIdx(
                                        8,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: FormPath(`mnist::Task`, `TypeAlias`),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                FormSynNodePath(`mnist::Task`, `TypeAlias`, (0)),
                            ),
                        ),
                        ident: `Task`,
                        visibility: Scope::PubUnder(
                            `mnist`,
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    FormSynNodePath(`mnist::TASK`, `Static`, (0)),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist`,
                                ),
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `TASK`,
                                    token_idx: TokenIdx(
                                        12,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: FormPath(`mnist::TASK`, `Static`),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                FormSynNodePath(`mnist::TASK`, `Static`, (0)),
                            ),
                        ),
                        ident: `TASK`,
                        visibility: Scope::PubUnder(
                            `mnist`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `Task`,
                        visible_scope: Scope::PubUnder(
                            `mnist`,
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`mnist::Task`, `TypeAlias`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `TASK`,
                        visible_scope: Scope::PubUnder(
                            `mnist`,
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`mnist::TASK`, `Static`),
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [
                    OnceUseRule {
                        ast_idx: 0,
                        use_expr_idx: 2,
                        visibility: Scope::PubUnder(
                            `mnist`,
                        ),
                        variant: OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::Ident(
                                IdentToken {
                                    ident: `mnist`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                            ),
                            children: ArenaIdxRange(
                                1..2,
                            ),
                        },
                        parent: None,
                        state: UseOneRuleState::Erroneous,
                    },
                ],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [
                EntityTreeError::Original(
                    OriginalEntityTreeError::UnresolvedRootIdent(
                        IdentToken {
                            ident: `mnist`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                ),
            ],
        },
    ],
    principal_item_path_expr_arena: Arena {
        data: [],
    },
}
```