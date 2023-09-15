Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: SubmodulePath(
                                `mnist_classifier::digits::zero`,
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
                                `mnist_classifier::digits::one`,
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
                                `mnist_classifier::digits::six`,
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
                                `mnist_classifier::digits::three`,
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
                                `mnist_classifier::digits::four`,
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
                                `mnist_classifier::digits::five`,
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
                                `mnist_classifier::digits::seven`,
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
                                `mnist_classifier::digits::eight`,
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
                                `mnist_classifier::digits::nine`,
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
                                `mnist_classifier::digits::two`,
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 2,
                rule_idx: OnceUseRuleIdx(
                    0,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits::one`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: OnceUseRuleIdx(
                    10,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                            ),
                        },
                    ),
                },
            },
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
                            module_path: `mnist_classifier::digits`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 4,
                rule_idx: OnceUseRuleIdx(
                    11,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits::six`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 3,
                rule_idx: OnceUseRuleIdx(
                    19,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                            ),
                        },
                    ),
                },
            },
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
                            module_path: `mnist_classifier::digits`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 7,
                rule_idx: OnceUseRuleIdx(
                    12,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits::zero`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 6,
                rule_idx: OnceUseRuleIdx(
                    20,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            ),
                        },
                    ),
                },
            },
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
                            module_path: `mnist_classifier::digits`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 10,
                rule_idx: OnceUseRuleIdx(
                    13,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits::two`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 9,
                rule_idx: OnceUseRuleIdx(
                    21,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                            ),
                        },
                    ),
                },
            },
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
                            module_path: `mnist_classifier::digits`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 13,
                rule_idx: OnceUseRuleIdx(
                    14,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits::three`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 12,
                rule_idx: OnceUseRuleIdx(
                    22,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                            ),
                        },
                    ),
                },
            },
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
                            module_path: `mnist_classifier::digits`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 16,
                rule_idx: OnceUseRuleIdx(
                    15,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits::five`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 15,
                rule_idx: OnceUseRuleIdx(
                    23,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 20,
                rule_idx: OnceUseRuleIdx(
                    6,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 19,
                rule_idx: OnceUseRuleIdx(
                    16,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits::seven`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 18,
                rule_idx: OnceUseRuleIdx(
                    24,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 23,
                rule_idx: OnceUseRuleIdx(
                    7,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 22,
                rule_idx: OnceUseRuleIdx(
                    17,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits::eight`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 21,
                rule_idx: OnceUseRuleIdx(
                    25,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 26,
                rule_idx: OnceUseRuleIdx(
                    8,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 25,
                rule_idx: OnceUseRuleIdx(
                    18,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits::nine`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 24,
                rule_idx: OnceUseRuleIdx(
                    26,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 28,
                rule_idx: OnceUseRuleIdx(
                    9,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)