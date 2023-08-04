Ok(
    EntitySynTreeSheet {
        module_path: `std::ops`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`std::ops::Add`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `std::ops`,
                            ),
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Add`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                items: Some(
                                    TraitItems {
                                        ast_idx_range: ArenaIdxRange(
                                            0..2,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Trait(
                            TraitSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitPath(`std::ops::Add`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Add`,
                    visibility: Scope::PubUnder(
                        `std::ops`,
                    ),
                },
            ],
        },
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Add`,
                    visibility: Scope::PubUnder(
                        `std::ops`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajarItemPath::Trait(
                            TraitPath(`std::ops::Add`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`std::ops::Add`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `std::ops`,
                            ),
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Add`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                items: Some(
                                    TraitItems {
                                        ast_idx_range: ArenaIdxRange(
                                            0..2,
                                        ),
                                    },
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