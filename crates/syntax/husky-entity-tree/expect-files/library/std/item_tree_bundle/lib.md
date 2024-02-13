EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
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
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `prelude`,
                        visibility: Scope::PubUnder(
                            `std`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `std::prelude`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `logic`,
                        visibility: Scope::PubUnder(
                            `std`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `std::logic`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `ops`,
                        visibility: Scope::PubUnder(
                            `std`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `std::ops`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `std::prelude`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [],
            },
            item_symbol_table: EntitySymbolTable(
                [],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `std::logic`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [],
            },
            item_symbol_table: EntitySymbolTable(
                [],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [
                    OnceUseRule {
                        ast_idx: 1,
                        use_expr_idx: 3,
                        visibility: Scope::Pub,
                        variant: OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::Ident(
                                IdentToken {
                                    ident: `core`,
                                    token_idx: TokenIdx(
                                        3,
                                    ),
                                },
                            ),
                            children: ArenaIdxRange(
                                2..3,
                            ),
                        },
                        parent: None,
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::UniversalPrelude {
                                    item_path: PrincipalEntityPath::Module(
                                        `core`,
                                    ),
                                },
                            ),
                        },
                    },
                    OnceUseRule {
                        ast_idx: 1,
                        use_expr_idx: 2,
                        visibility: Scope::Pub,
                        variant: OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::Ident(
                                IdentToken {
                                    ident: `logic`,
                                    token_idx: TokenIdx(
                                        5,
                                    ),
                                },
                            ),
                            children: ArenaIdxRange(
                                1..2,
                            ),
                        },
                        parent: Some(
                            (
                                MajorEntityPath::Module(
                                    `core`,
                                ),
                                EntitySymbol::UniversalPrelude {
                                    item_path: PrincipalEntityPath::Module(
                                        `core`,
                                    ),
                                },
                            ),
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::PackageDependency {
                                    item_path: PrincipalEntityPath::Module(
                                        `core::logic`,
                                    ),
                                },
                            ),
                        },
                    },
                ],
            ),
            use_all_rules: UseAllRules(
                [
                    UseAllRule {
                        parent_module_path: `core::logic`,
                        is_same_crate: false,
                        ast_idx: 1,
                        use_expr_idx: 1,
                        visibility: Scope::Pub,
                        progress: Ok(
                            0,
                        ),
                    },
                ],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `std::ops`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Trait(
                                    TraitSynNodePath(
                                        ItemSynNodePathId(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `std::ops`,
                                ),
                                ast_idx: 4,
                                ident_token: IdentToken {
                                    ident: `Add`,
                                    token_idx: TokenIdx(
                                        8,
                                    ),
                                },
                                block: DefnBlock::Trait {
                                    path: TraitPath(`std::ops::Add`),
                                    items: Some(
                                        TraitItems {
                                            ast_idx_range: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Trait(
                                TraitSynNodePath(
                                    ItemSynNodePathId(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        ident: `Add`,
                        visibility: Scope::PubUnder(
                            `std::ops`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `Add`,
                        visibility: Scope::PubUnder(
                            `std::ops`,
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Trait(
                                TraitPath(`std::ops::Add`),
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
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