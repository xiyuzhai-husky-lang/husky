Ok(
    EntityTreeSheet {
        module_path: `std::ops`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Trait(
                                TraitNodePath {
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
                                        value: 52,
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
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Trait(
                            TraitNodePath {
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
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Add`,
                    visibility: Scope::PubUnder(
                        `std::ops`,
                    ),
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Trait(
                            TraitPath(`std::ops::Add`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Trait(
                                TraitNodePath {
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
                                        value: 52,
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
        impl_block_node_table: [],
        once_use_rules: OnceUseRules(
            [],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [],
        ),
        errors: [],
    },
)