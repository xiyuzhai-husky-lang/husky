```rust
EntityTreeSheet {
    module_path: ModulePath(`syntax_basics::defn::major_item::ty::enum_ty`),
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::defn::major_item::ty::enum_ty`),
                        ),
                        ast_idx: 5,
                        ident_token: IdentToken {
                            ident: `A`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                            variants: Some(
                                TypeVariants {
                                    ast_idx_range: ArenaIdxRange(
                                        0..5,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Type(
                                        TypeSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `A`,
                visibility: Scope::PubUnder(
                    ModulePath(`syntax_basics::defn::major_item::ty::enum_ty`),
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `A`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`syntax_basics::defn::major_item::ty::enum_ty`),
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Type(
                        TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
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