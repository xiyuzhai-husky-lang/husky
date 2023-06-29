Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::one`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            ast_idx: 62,
                            ident_token: IdentToken {
                                ident: `one_fermi_match`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `one_fermi_match`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::one`,
                    ),
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 63,
                            ident_token: IdentToken {
                                ident: `is_one`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `is_one`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            ast_idx: 64,
                            ident_token: IdentToken {
                                ident: `upmost`,
                                token_idx: TokenIdx(
                                    488,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `upmost`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::one`,
                    ),
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            ast_idx: 65,
                            ident_token: IdentToken {
                                ident: `downmost`,
                                token_idx: TokenIdx(
                                    517,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `downmost`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::one`,
                    ),
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            ast_idx: 66,
                            ident_token: IdentToken {
                                ident: `hat`,
                                token_idx: TokenIdx(
                                    551,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `hat`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::one`,
                    ),
                },
            ],
        },
        use_one_trackers: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 61,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::one`,
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