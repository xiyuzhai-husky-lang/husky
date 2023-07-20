Ok(
    EntityTreePresheet {
        module_path: `natural_number_game`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `Nat`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 55,
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
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`natural_number_game::Nat`, `Inductive`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Nat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`natural_number_game::OddNat`, `Structure`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 9,
                            ident_token: IdentToken {
                                ident: `OddNat`,
                                token_idx: TokenIdx(
                                    83,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 56,
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
                                    path: TypePath(`natural_number_game::OddNat`, `Structure`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `OddNat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 10,
                            ident_token: IdentToken {
                                ident: `EvenNat`,
                                token_idx: TokenIdx(
                                    112,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 57,
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
                                    path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `EvenNat`,
                    visibility: Scope::PubUnder(
                        `natural_number_game`,
                    ),
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