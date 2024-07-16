```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: ModulePath(`mnist`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`mnist::Task`, `TypeVar`, (0)),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `Task`,
                                    token_idx: TokenIdx(
                                        10,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: MajorFormPath(`mnist::Task`, `TypeVar`),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist::Task`, `TypeVar`, (0)),
                            ),
                        ),
                        ident: `Task`,
                        visibility: Scope::Pub,
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`mnist::TASK`, `StaticVar`, (0)),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `TASK`,
                                    token_idx: TokenIdx(
                                        16,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: MajorFormPath(`mnist::TASK`, `StaticVar`),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist::TASK`, `StaticVar`, (0)),
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
                                MajorFormPath(`mnist::Task`, `TypeVar`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `TASK`,
                        visible_scope: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`mnist::TASK`, `StaticVar`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `MnistTask`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`mnist`),
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
                                    ModulePath(`mnist`),
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
                            ModulePath(`mnist`),
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
                                        ModulePath(`mnist`),
                                    ),
                                },
                            ),
                        },
                    },
                    OnceUseRule {
                        ast_idx: 0,
                        use_expr_idx: 1,
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist`),
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
                                    ModulePath(`mnist`),
                                ),
                                EntitySymbol::PackageDependencyOrSelfLib {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`mnist`),
                                    ),
                                },
                            ),
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::PackageDependencyOrSelfLib {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`mnist::task`),
                                    ),
                                },
                            ),
                        },
                    },
                    OnceUseRule {
                        ast_idx: 0,
                        use_expr_idx: 0,
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist`),
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
                                    ModulePath(`mnist::task`),
                                ),
                                EntitySymbol::PackageDependencyOrSelfLib {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`mnist::task`),
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
        },
    ],
    principal_item_path_expr_arena: Arena {
        data: [],
    },
}
```