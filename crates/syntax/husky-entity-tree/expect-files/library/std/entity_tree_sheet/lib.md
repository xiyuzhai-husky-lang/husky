Ok(
    EntityTreeSheet {
        module_path: `std`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::Submodule(
                        SubmoduleNode {
                            node_path: SubmoduleNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: `std::prelude`,
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
                    node_path: EntityNodePath::Submodule(
                        SubmoduleNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: `std::prelude`,
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
                    node: EntityNode::Submodule(
                        SubmoduleNode {
                            node_path: SubmoduleNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: `std::logic`,
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
                    node_path: EntityNodePath::Submodule(
                        SubmoduleNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: `std::logic`,
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
                    node: EntityNode::Submodule(
                        SubmoduleNode {
                            node_path: SubmoduleNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: `std::ops`,
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
                    node_path: EntityNodePath::Submodule(
                        SubmoduleNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: `std::ops`,
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
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `prelude`,
                    visibility: Scope::PubUnder(
                        `std`,
                    ),
                    symbol: EntitySymbol::Submodule {
                        submodule_path: `std::prelude`,
                        node: SubmoduleNode {
                            node_path: SubmoduleNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: `std::prelude`,
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
                        submodule_path: `std::logic`,
                        node: SubmoduleNode {
                            node_path: SubmoduleNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: `std::logic`,
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
                        submodule_path: `std::ops`,
                        node: SubmoduleNode {
                            node_path: SubmoduleNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: `std::ops`,
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
        impl_block_node_table: [],
        use_expr_rules: UseExprRules(
            [],
        ),
        use_all_rules: UseAllRules(
            [],
        ),
        errors: [],
    },
)