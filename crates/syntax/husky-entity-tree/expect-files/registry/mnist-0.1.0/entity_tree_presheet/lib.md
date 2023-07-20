Ok(
    EntityTreePresheet {
        module_path: `mnist`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 10,
                            ident_token: IdentToken {
                                ident: `MnistLabel`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 60,
                                    },
                                ),
                                variants: Some(
                                    TypeVariants {
                                        ast_idx_range: ArenaIdxRange(
                                            0..10,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist::MnistLabel`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `MnistLabel`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 12,
                            ident_token: IdentToken {
                                ident: `BinaryImage28`,
                                token_idx: TokenIdx(
                                    32,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 61,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `BinaryImage28`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist::input`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 13,
                            ident_token: IdentToken {
                                ident: `input`,
                                token_idx: TokenIdx(
                                    41,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 78,
                                    },
                                ),
                                body: None,
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist::input`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `input`,
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