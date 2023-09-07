Ok(
    EntitySynTreePresheet {
        module_path: `core::logic`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::logic::Prop`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `Prop`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::logic::Prop`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Prop`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::logic::LogicAnd`, `Structure`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `LogicAnd`,
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 8,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::logic::LogicAnd`, `Structure`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `LogicAnd`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::logic::LogicOr`, `Inductive`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 6,
                            ident_token: IdentToken {
                                ident: `LogicOr`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 9,
                                    },
                                ),
                                variants: Some(
                                    TypeVariants {
                                        ast_idx_range: ArenaIdxRange(
                                            1..3,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::logic::LogicOr`, `Inductive`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `LogicOr`,
                    visibility: Scope::Pub,
                },
            ],
        },
        use_one_trackers: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 3,
                    use_expr_idx: 2,
                    visibility: Scope::PubUnder(
                        `core::logic`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            1..2,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
            ],
        ),
        use_all_trackers: UseAllModuleSymbolsRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            4,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    3,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 1,
                            },
                        ),
                    },
                ),
            ],
        },
        errors: [],
    },
)