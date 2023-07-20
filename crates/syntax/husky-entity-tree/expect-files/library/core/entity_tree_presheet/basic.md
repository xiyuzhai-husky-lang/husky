Ok(
    EntityTreePresheet {
        module_path: `core::basic`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::bool`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `bool`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 2,
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
                                    path: TypePath(`core::basic::bool`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `bool`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::never`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `never`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 3,
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
                                    path: TypePath(`core::basic::never`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `never`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::unit`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `unit`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 4,
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
                                    path: TypePath(`core::basic::unit`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `unit`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::Trait`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Trait`,
                                token_idx: TokenIdx(
                                    14,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 5,
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
                                    path: TypePath(`core::basic::Trait`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Trait`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::Module`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `Module`,
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 6,
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
                                    path: TypePath(`core::basic::Module`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Module`,
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