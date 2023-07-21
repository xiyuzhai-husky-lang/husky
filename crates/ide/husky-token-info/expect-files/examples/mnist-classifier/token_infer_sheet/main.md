Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `mnist_classifier::connected_component`,
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `mnist_classifier::raw_contour`,
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `mnist_classifier::geom2d`,
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `mnist_classifier::line_segment_sketch`,
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `mnist_classifier::fermi`,
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `mnist_classifier::digits`,
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `mnist_classifier::major`,
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 2,
                rule_idx: OnceUseRuleIdx(
                    0,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: OnceUseRuleIdx(
                    8,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::major`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::major`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 17,
                                ident_token: IdentToken {
                                    ident: `major`,
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 5,
                rule_idx: OnceUseRuleIdx(
                    1,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 4,
                rule_idx: OnceUseRuleIdx(
                    9,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 16,
                                ident_token: IdentToken {
                                    ident: `digits`,
                                    token_idx: TokenIdx(
                                        11,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 8,
                rule_idx: OnceUseRuleIdx(
                    2,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 7,
                rule_idx: OnceUseRuleIdx(
                    10,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::fermi`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::fermi`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 15,
                                ident_token: IdentToken {
                                    ident: `fermi`,
                                    token_idx: TokenIdx(
                                        9,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 11,
                rule_idx: OnceUseRuleIdx(
                    3,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 10,
                rule_idx: OnceUseRuleIdx(
                    11,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::raw_contour`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::raw_contour`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 12,
                                ident_token: IdentToken {
                                    ident: `raw_contour`,
                                    token_idx: TokenIdx(
                                        3,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 14,
                rule_idx: OnceUseRuleIdx(
                    4,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 13,
                rule_idx: OnceUseRuleIdx(
                    12,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::line_segment_sketch`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::line_segment_sketch`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 14,
                                ident_token: IdentToken {
                                    ident: `line_segment_sketch`,
                                    token_idx: TokenIdx(
                                        7,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 17,
                rule_idx: OnceUseRuleIdx(
                    5,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 16,
                rule_idx: OnceUseRuleIdx(
                    13,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::connected_component`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::connected_component`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 11,
                                ident_token: IdentToken {
                                    ident: `connected_component`,
                                    token_idx: TokenIdx(
                                        1,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 19,
                rule_idx: OnceUseRuleIdx(
                    6,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::PackageDependency {
                            entity_path: PrincipalEntityPath::Module(
                                `malamute`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 21,
                rule_idx: OnceUseRuleIdx(
                    7,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::PackageDependency {
                            entity_path: PrincipalEntityPath::Module(
                                `mnist`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)