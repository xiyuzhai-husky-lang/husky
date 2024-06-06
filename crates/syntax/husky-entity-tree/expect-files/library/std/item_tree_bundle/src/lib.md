```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: ModulePath(`std`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`std::prelude),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`std`),
                                ),
                                ast_idx: 0,
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
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`std::prelude),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `prelude`,
                        visibility: Scope::PubUnder(
                            ModulePath(`std`),
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`std::logic),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`std`),
                                ),
                                ast_idx: 1,
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
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`std::logic),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `logic`,
                        visibility: Scope::PubUnder(
                            ModulePath(`std`),
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`std::ops),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`std`),
                                ),
                                ast_idx: 2,
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
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`std::ops),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `ops`,
                        visibility: Scope::PubUnder(
                            ModulePath(`std`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `prelude`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`std`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`std::prelude),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `logic`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`std`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`std::logic),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `ops`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`std`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`std::ops),
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
            module_path: ModulePath(`std::prelude`),
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
            module_path: ModulePath(`std::logic`),
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
                        ast_idx: 0,
                        use_expr_idx: 2,
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
                                1..2,
                            ),
                        },
                        parent: None,
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::UniversalPrelude {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`core`),
                                    ),
                                },
                            ),
                        },
                    },
                    OnceUseRule {
                        ast_idx: 0,
                        use_expr_idx: 1,
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
                                0..1,
                            ),
                        },
                        parent: Some(
                            (
                                MajorEntityPath::Module(
                                    ModulePath(`core`),
                                ),
                                EntitySymbol::UniversalPrelude {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`core`),
                                    ),
                                },
                            ),
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::PackageDependencyOrSelfLib {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`core::logic`),
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
                        parent_module_path: ModulePath(`core::logic`),
                        is_same_crate: false,
                        ast_idx: 0,
                        use_expr_idx: 0,
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
            module_path: ModulePath(`std::ops`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Trait(
                                    TraitSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Trait(
                                                    TraitSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TraitPath(`std::ops::Add`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`std::ops`),
                                ),
                                ast_idx: 3,
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
                                                0..2,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Trait(
                                TraitSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Trait(
                                                TraitSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TraitPath(`std::ops::Add`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `Add`,
                        visibility: Scope::PubUnder(
                            ModulePath(`std::ops`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `Add`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`std::ops`),
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
```