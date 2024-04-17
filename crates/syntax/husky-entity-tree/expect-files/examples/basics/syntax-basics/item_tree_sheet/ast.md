```rust
EntityTreeSheet {
    module_path: `syntax_basics::ast`,
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
                                            maybe_ambiguous_item_path: SubmoduleItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 5,
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
                            `syntax_basics::ast`,
                        ),
                        ast_idx: 0,
                        ident_token: IdentToken {
                            ident: `submodule_name`,
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
                                        maybe_ambiguous_item_path: SubmoduleItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 5,
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
                ident: `submodule_name`,
                visibility: Scope::PubUnder(
                    `syntax_basics::ast`,
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `submodule_name`,
                visibility: Scope::PubUnder(
                    `syntax_basics::ast`,
                ),
                symbol: EntitySymbol::Submodule {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 5,
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