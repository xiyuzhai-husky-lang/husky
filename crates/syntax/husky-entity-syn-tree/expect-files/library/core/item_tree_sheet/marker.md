Ok(
    EntitySynTreeSheet {
        module_path: `core::marker`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::marker::Copy`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Copy`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                items: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Trait(
                            TraitSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitPath(`core::marker::Copy`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Copy`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::marker::Sized`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `Sized`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 8,
                                    },
                                ),
                                items: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Trait(
                            TraitSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitPath(`core::marker::Sized`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Sized`,
                    visibility: Scope::Pub,
                },
            ],
        },
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Copy`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Trait(
                            TraitPath(`core::marker::Copy`),
                        ),
                        node: ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::marker::Copy`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Copy`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                items: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `Sized`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Trait(
                            TraitPath(`core::marker::Sized`),
                        ),
                        node: ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::marker::Sized`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `Sized`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 8,
                                    },
                                ),
                                items: None,
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