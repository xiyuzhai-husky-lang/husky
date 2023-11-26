EntitySynTreeSheet {
    module_path: `std`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::Submodule(
                    SubmoduleSynNodeData {
                        syn_node_path: SubmoduleSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmodulePath(
                                    `std::prelude`,
                                ),
                                disambiguator: 0,
                            },
                        },
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
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `std::prelude`,
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ident: `prelude`,
                visibility: Scope::PubUnder(
                    `std`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::Submodule(
                    SubmoduleSynNodeData {
                        syn_node_path: SubmoduleSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmodulePath(
                                    `std::logic`,
                                ),
                                disambiguator: 0,
                            },
                        },
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
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `std::logic`,
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ident: `logic`,
                visibility: Scope::PubUnder(
                    `std`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::Submodule(
                    SubmoduleSynNodeData {
                        syn_node_path: SubmoduleSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmodulePath(
                                    `std::ops`,
                                ),
                                disambiguator: 0,
                            },
                        },
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
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `std::ops`,
                            ),
                            disambiguator: 0,
                        },
                    },
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
                    submodule_path: SubmodulePath(
                        `std::prelude`,
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `logic`,
                visibility: Scope::PubUnder(
                    `std`,
                ),
                symbol: EntitySymbol::Submodule {
                    submodule_path: SubmodulePath(
                        `std::logic`,
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `ops`,
                visibility: Scope::PubUnder(
                    `std`,
                ),
                symbol: EntitySymbol::Submodule {
                    submodule_path: SubmodulePath(
                        `std::ops`,
                    ),
                },
            },
        ],
    ),
    impl_block_syn_node_table: [],
    once_use_rules: OnceUseRules(
        [],
    ),
    use_all_rules: UseAllModuleSymbolsRules(
        [],
    ),
    errors: [],
}