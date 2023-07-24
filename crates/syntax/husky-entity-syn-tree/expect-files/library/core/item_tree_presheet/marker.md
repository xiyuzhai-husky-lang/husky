Ok(
    EntitySynTreePresheet {
        module_path: `core::marker`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntitySynNode::ModuleItem(
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
                    syn_node_path: EntitySynNodePath::ModuleItem(
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
                    node: EntitySynNode::ModuleItem(
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
                    syn_node_path: EntitySynNodePath::ModuleItem(
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