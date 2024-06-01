```rust
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
                        visibility: Scope::Pub,
                        ast_idx: 1,
                        ident_token: IdentToken {
                            ident: `Task`,
                            token_idx: TokenIdx(
                                9,
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
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            FormSynNodePath(`mnist::TASK`, `Static`, (0)),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        ident_token: IdentToken {
                            ident: `TASK`,
                            token_idx: TokenIdx(
                                14,
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
                visibility: Scope::Pub,
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `Task`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        FormPath(`mnist::Task`, `TypeAlias`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `TASK`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        FormPath(`mnist::TASK`, `Static`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `MnistTask`,
                visible_scope: Scope::PubUnder(
                    `mnist`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::PackageDependencyOrSelfLib {
                            item_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist::task::MnistTask`, `Extern`),
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist::task::MnistTask`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist`,
                        ),
                        ast_idx: 0,
                        use_expr_idx: 0,
                    },
                ),
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
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::PackageDependencyOrSelfLib {
                            item_path: PrincipalEntityPath::Module(
                                `mnist`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 0,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    `mnist`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `task`,
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        0..1,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            `mnist`,
                        ),
                        EntitySymbol::PackageDependencyOrSelfLib {
                            item_path: PrincipalEntityPath::Module(
                                `mnist`,
                            ),
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::PackageDependencyOrSelfLib {
                            item_path: PrincipalEntityPath::Module(
                                `mnist::task`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 0,
                use_expr_idx: 0,
                visibility: Scope::PubUnder(
                    `mnist`,
                ),
                variant: OnceUseRuleVariant::IdentLeaf {
                    ident_token: IdentToken {
                        ident: `MnistTask`,
                        token_idx: TokenIdx(
                            6,
                        ),
                    },
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            `mnist::task`,
                        ),
                        EntitySymbol::PackageDependencyOrSelfLib {
                            item_path: PrincipalEntityPath::Module(
                                `mnist::task`,
                            ),
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::PackageDependencyOrSelfLib {
                            item_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist::task::MnistTask`, `Extern`),
                                ),
                            ),
                        },
                    ),
                },
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    errors: [],
}
```