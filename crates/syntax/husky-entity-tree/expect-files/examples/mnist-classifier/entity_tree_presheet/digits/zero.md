Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::zero`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            ast_idx: 26,
                            ident_token: IdentToken {
                                ident: `open_one_match`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 51,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            0..1,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `open_one_match`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            ast_idx: 27,
                            ident_token: IdentToken {
                                ident: `almost_closed`,
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 52,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            1..3,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `almost_closed`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 28,
                            ident_token: IdentToken {
                                ident: `is_zero`,
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 53,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            11..25,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `is_zero`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                },
            ],
        },
        use_one_trackers: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 25,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Super(
                            SuperToken {
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
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
                            3,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Super(
                            SuperToken {
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    2,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 0,
                            },
                        ),
                    },
                ),
            ],
        },
        errors: [],
    },
)