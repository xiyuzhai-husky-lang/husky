Ok(
    EntitySynTreePresheet {
        module_path: `core::visual`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Trait(
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::visual::Visualize`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `Visualize`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Trait {
                                path: TraitPath(
                                    Id {
                                        value: 25,
                                    },
                                ),
                                items: Some(
                                    TraitItems {
                                        ast_idx_range: ArenaIdxRange(
                                            0..1,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Trait(
                            TraitSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitPath(`core::visual::Visualize`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Visualize`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::visual::Html`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Html`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 36,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::visual::Html`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Html`,
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