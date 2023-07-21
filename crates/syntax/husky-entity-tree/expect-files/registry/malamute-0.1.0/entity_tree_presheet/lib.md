Ok(
    EntityTreePresheet {
        module_path: `malamute`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntitySynNode::ModuleItem(
                        ModuleItemSynNode {
                            node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 6,
                            ident_token: IdentToken {
                                ident: `OneVsAll`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 58,
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
                    node_path: EntitySynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `OneVsAll`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntitySynNode::ModuleItem(
                        ModuleItemSynNode {
                            node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 7,
                            ident_token: IdentToken {
                                ident: `OneVsAllResult`,
                                token_idx: TokenIdx(
                                    17,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 59,
                                    },
                                ),
                                variants: Some(
                                    TypeVariants {
                                        ast_idx_range: ArenaIdxRange(
                                            2..5,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    node_path: EntitySynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `OneVsAllResult`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntitySynNode::ModuleItem(
                        ModuleItemSynNode {
                            node_path: ModuleItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 9,
                            ident_token: IdentToken {
                                ident: `narrow_down`,
                                token_idx: TokenIdx(
                                    61,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 77,
                                    },
                                ),
                                body: None,
                            },
                        },
                    ),
                    node_path: EntitySynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `narrow_down`,
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