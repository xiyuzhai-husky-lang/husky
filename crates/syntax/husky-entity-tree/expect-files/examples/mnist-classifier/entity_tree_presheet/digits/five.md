Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::five`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Fugitive(
                                FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `is_five`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Fugitive(
                            FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `is_five`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                },
            ],
        },
        use_one_trackers: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 1,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::five`,
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