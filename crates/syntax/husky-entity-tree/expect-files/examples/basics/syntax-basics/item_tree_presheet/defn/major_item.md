```rust
EntityTreePresheet {
    module_path: `syntax_basics::defn::major_item`,
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
                                                        value: 13,
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
                            `syntax_basics::defn::major_item`,
                        ),
                        ast_idx: 1,
                        ident_token: IdentToken {
                            ident: `ty`,
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
                                                    value: 13,
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
                ident: `ty`,
                visibility: Scope::PubUnder(
                    `syntax_basics::defn::major_item`,
                ),
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    use_expr_arena: Arena {
        data: [],
    },
}
```