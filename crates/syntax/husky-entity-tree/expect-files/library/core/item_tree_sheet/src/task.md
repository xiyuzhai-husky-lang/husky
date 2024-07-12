```rust
EntityTreeSheet {
    module_path: ModulePath(`core::task`),
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
                                                    maybe_ambiguous_item_path: TraitPath(`core::task::IsTask`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        ident_token: IdentToken {
                            ident: `IsTask`,
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                        block: DefnBlock::Trait {
                            path: TraitPath(`core::task::IsTask`),
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
                                                maybe_ambiguous_item_path: TraitPath(`core::task::IsTask`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `IsTask`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`core::task::Task`, `TypeVar`, (0)),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 3,
                        ident_token: IdentToken {
                            ident: `Task`,
                            token_idx: TokenIdx(
                                18,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`core::task::Task`, `TypeVar`),
                            body: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`core::task::Task`, `TypeVar`, (0)),
                    ),
                ),
                ident: `Task`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`core::task::TASK`, `StaticVar`, (0)),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 4,
                        ident_token: IdentToken {
                            ident: `TASK`,
                            token_idx: TokenIdx(
                                22,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`core::task::TASK`, `StaticVar`),
                            body: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`core::task::TASK`, `StaticVar`, (0)),
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
                ident: `IsTask`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Trait(
                        TraitPath(`core::task::IsTask`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `Task`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        FormPath(`core::task::Task`, `TypeVar`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `TASK`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        FormPath(`core::task::TASK`, `StaticVar`),
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
}
```