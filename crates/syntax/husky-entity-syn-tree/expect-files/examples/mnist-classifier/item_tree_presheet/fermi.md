Ok(
    EntitySynTreePresheet {
        module_path: `mnist_classifier::fermi`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntitySynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 22,
                            ident_token: IdentToken {
                                ident: `FermiMatchResult`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: EntitySynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `FermiMatchResult`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntitySynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 24,
                            ident_token: IdentToken {
                                ident: `fermi_match`,
                                token_idx: TokenIdx(
                                    147,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            17..21,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: EntitySynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `fermi_match`,
                    visibility: Scope::Pub,
                },
            ],
        },
        use_one_trackers: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 21,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
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
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
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