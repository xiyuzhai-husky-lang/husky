```rust
EntityTreeSheet {
    module_path: `syntax_basics::defn::major_item::ty`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::Submodule(
                    SubmoduleSynNode {
                        syn_node_path: SubmoduleSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::Submodule(
                                    SubmoduleSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmoduleItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 8,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            },
                        ),
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn::major_item::ty`,
                        ),
                        ast_idx: 0,
                        ident_token: IdentToken {
                            ident: `enum_ty`,
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
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmoduleItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 8,
                                                },
                                            ),
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        },
                    ),
                ),
                ident: `enum_ty`,
                visibility: Scope::PubUnder(
                    `syntax_basics::defn::major_item::ty`,
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `enum_ty`,
                visibility: Scope::PubUnder(
                    `syntax_basics::defn::major_item::ty`,
                ),
                symbol: EntitySymbol::Submodule {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 8,
                            },
                        ),
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