Ok(
    EntityTreeSheet {
        module_path: `natural_number_game`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `Nat`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Nat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`natural_number_game::OddNat`, `Structure`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 9,
                            ident_token: IdentToken {
                                ident: `OddNat`,
                                token_idx: TokenIdx(
                                    83,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`natural_number_game::OddNat`, `Structure`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `OddNat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 10,
                            ident_token: IdentToken {
                                ident: `EvenNat`,
                                token_idx: TokenIdx(
                                    112,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `EvenNat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                },
            ],
        },
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Nat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`natural_number_game::Nat`, `Inductive`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `Nat`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `OddNat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`natural_number_game::OddNat`, `Structure`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`natural_number_game::OddNat`, `Structure`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 9,
                            ident_token: IdentToken {
                                ident: `OddNat`,
                                token_idx: TokenIdx(
                                    83,
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `EvenNat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`natural_number_game::EvenNat`, `Structure`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 10,
                            ident_token: IdentToken {
                                ident: `EvenNat`,
                                token_idx: TokenIdx(
                                    112,
                                ),
                            },
                        },
                    },
                },
            ],
        ),
        impl_block_node_table: [
            (
                ImplBlockNodePath::TypeImplBlock(
                    TypeImplBlockNodePath {
                        path: TypeImplBlockPath {
                            module_path: `natural_number_game`,
                            ty_path: TypePath(`natural_number_game::Nat`, `Inductive`),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockNode::TypeImplBlock(
                    TypeImplBlockNode {
                        node_path: TypeImplBlockNodePath {
                            path: TypeImplBlockPath {
                                module_path: `natural_number_game`,
                                ty_path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 6,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                9,
                            ),
                        },
                        ty_expr: 0,
                        items: TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                2..5,
                            ),
                        },
                    },
                ),
            ),
        ],
        use_expr_rules: UseExprRules(
            [],
        ),
        use_all_rules: UseAllRules(
            [],
        ),
        errors: [],
    },
)