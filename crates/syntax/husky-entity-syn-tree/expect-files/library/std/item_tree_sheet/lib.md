Ok(
    EntitySynTreeSheet {
        module_path: `std`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntitySynNode::Submodule(
                        SubmoduleSynNode {
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
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `prelude`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                    syn_node_path: EntitySynNodePath::Submodule(
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
                EntityNodeEntry {
                    node: EntitySynNode::Submodule(
                        SubmoduleSynNode {
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
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `logic`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
                    ),
                    syn_node_path: EntitySynNodePath::Submodule(
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
                EntityNodeEntry {
                    node: EntitySynNode::Submodule(
                        SubmoduleSynNode {
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
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `ops`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                    syn_node_path: EntitySynNodePath::Submodule(
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
                        node: SubmoduleSynNode {
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
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `prelude`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
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
                        node: SubmoduleSynNode {
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
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `logic`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
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
                        node: SubmoduleSynNode {
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
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `ops`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
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
    },
)