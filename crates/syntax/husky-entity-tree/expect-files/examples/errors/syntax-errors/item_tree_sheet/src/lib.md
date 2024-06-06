```rust
EntityTreeSheet {
    module_path: ModulePath(`syntax_errors`),
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
                                            maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_errors::ast),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            },
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_errors`),
                        ),
                        ast_idx: 0,
                        ident_token: IdentToken {
                            ident: `ast`,
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
                                        maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_errors::ast),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        },
                    ),
                ),
                ident: `ast`,
                visibility: Scope::PubUnder(
                    ModulePath(`syntax_errors`),
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
                                            maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_errors::uses),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            },
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_errors`),
                        ),
                        ast_idx: 1,
                        ident_token: IdentToken {
                            ident: `uses`,
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
                                        maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_errors::uses),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        },
                    ),
                ),
                ident: `uses`,
                visibility: Scope::PubUnder(
                    ModulePath(`syntax_errors`),
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `ast`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`syntax_errors`),
                ),
                symbol: EntitySymbol::Submodule {
                    submodule_item_path: SubmoduleItemPath(`syntax_errors::ast),
                },
            },
            EntitySymbolEntry {
                ident: `uses`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`syntax_errors`),
                ),
                symbol: EntitySymbol::Submodule {
                    submodule_item_path: SubmoduleItemPath(`syntax_errors::uses),
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