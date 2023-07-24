Ok(
    EntitySynTreeSheet {
        module_path: `core::fmt`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntitySynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::fmt::Debug`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Debug`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                items: None,
                            },
                        },
                    ),
                    syn_node_path: EntitySynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Trait(
                            TraitSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitPath(`core::fmt::Debug`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Debug`,
                    visibility: Scope::Pub,
                },
            ],
        },
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Debug`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Trait(
                            TraitPath(`core::fmt::Debug`),
                        ),
                        node: ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::fmt::Debug`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Debug`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 6,
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