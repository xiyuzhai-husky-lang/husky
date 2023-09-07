Ok(
    EntitySynTreeSheet {
        module_path: `natural_number_game`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
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
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 67,
                                    },
                                ),
                                variants: Some(
                                    TypeVariants {
                                        ast_idx_range: ArenaIdxRange(
                                            0..2,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
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
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
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
                                    84,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 68,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
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
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
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
                                    113,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 69,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
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
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Nat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`natural_number_game::Nat`, `Inductive`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
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
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 67,
                                    },
                                ),
                                variants: Some(
                                    TypeVariants {
                                        ast_idx_range: ArenaIdxRange(
                                            0..2,
                                        ),
                                    },
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
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`natural_number_game::OddNat`, `Structure`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
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
                                    84,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 68,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `EvenNat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`natural_number_game::EvenNat`, `Structure`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
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
                                    113,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 69,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
            ],
        ),
        impl_block_syn_node_table: [
            (
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `natural_number_game`,
                            ty_path: TypePath(`natural_number_game::Nat`, `Inductive`),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TypeImplBlock(
                    TypeImplBlockSynNode {
                        syn_node_path: TypeImplBlockSynNodePath {
                            path: TypeImplBlockPath {
                                module_path: `natural_number_game`,
                                ty_path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 6,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                10,
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
        once_use_rules: OnceUseRules(
            [],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [],
        ),
        errors: [],
    },
)