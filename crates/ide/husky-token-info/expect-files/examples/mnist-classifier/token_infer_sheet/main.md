Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `mnist_classifier::connected_component`,
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `mnist_classifier::raw_contour`,
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `mnist_classifier::geom2d`,
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `mnist_classifier::fermi`,
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `mnist_classifier::digits`,
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `mnist_classifier::major`,
                            ),
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
                            submodule_path: SubmodulePath(
                                `mnist_classifier::major`,
                            ),
                            node: SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `mnist_classifier::major`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 16,
                                ident_token: IdentToken {
                                    ident: `major`,
                                    token_idx: TokenIdx(
                                        14,
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
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits`,
                            ),
                            node: SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `mnist_classifier::digits`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 15,
                                ident_token: IdentToken {
                                    ident: `digits`,
                                    token_idx: TokenIdx(
                                        12,
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
                            submodule_path: SubmodulePath(
                                `mnist_classifier::fermi`,
                            ),
                            node: SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `mnist_classifier::fermi`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 14,
                                ident_token: IdentToken {
                                    ident: `fermi`,
                                    token_idx: TokenIdx(
                                        10,
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
                            submodule_path: SubmodulePath(
                                `mnist_classifier::raw_contour`,
                            ),
                            node: SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `mnist_classifier::raw_contour`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 11,
                                ident_token: IdentToken {
                                    ident: `raw_contour`,
                                    token_idx: TokenIdx(
                                        4,
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
                            submodule_path: SubmodulePath(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            node: SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `mnist_classifier::line_segment_sketch`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 13,
                                ident_token: IdentToken {
                                    ident: `line_segment_sketch`,
                                    token_idx: TokenIdx(
                                        8,
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
                            submodule_path: SubmodulePath(
                                `mnist_classifier::connected_component`,
                            ),
                            node: SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 10,
                                ident_token: IdentToken {
                                    ident: `connected_component`,
                                    token_idx: TokenIdx(
                                        2,
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
                            item_path: PrincipalEntityPath::Module(
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
                            item_path: PrincipalEntityPath::Module(
                                `mnist`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::main`, `Val`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                MajorItem {
                    module_item_kind: Fugitive(
                        Val,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::MnistLabel`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                        ident: `Known`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::MnistLabel`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                        ident: `Four`,
                    },
                ),
            ),
            TokenInfo::None,
        ],
    },
)