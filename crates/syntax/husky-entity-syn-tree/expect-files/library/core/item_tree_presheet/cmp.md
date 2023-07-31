Ok(
    EntitySynTreePresheet {
        module_path: `core::cmp`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::cmp::PartialEq`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `PartialEq`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 2,
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
                                    path: TraitPath(`core::cmp::PartialEq`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `PartialEq`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::cmp::Eq`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `Eq`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 3,
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
                                    path: TraitPath(`core::cmp::Eq`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Eq`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::cmp::PartialOrd`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `PartialOrd`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 4,
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
                                    path: TraitPath(`core::cmp::PartialOrd`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `PartialOrd`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::cmp::Ord`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Ord`,
                                token_idx: TokenIdx(
                                    14,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 5,
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
                                    path: TraitPath(`core::cmp::Ord`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Ord`,
                    visibility: Scope::Pub,
                },
            ],
        },
        use_one_trackers: OnceUseRules(
            [],
        ),
        use_all_trackers: UseAllModuleSymbolsRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [],
        },
        errors: [],
    },
)