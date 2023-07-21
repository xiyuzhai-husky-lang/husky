Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::Submodule(
                    SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `mnist_classifier::digits::zero`,
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
                            path: `mnist_classifier::digits::one`,
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
                            path: `mnist_classifier::digits::six`,
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
                            path: `mnist_classifier::digits::three`,
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
                            path: `mnist_classifier::digits::four`,
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
                            path: `mnist_classifier::digits::five`,
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
                            path: `mnist_classifier::digits::seven`,
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
                            path: `mnist_classifier::digits::eight`,
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
                            path: `mnist_classifier::digits::nine`,
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
                            path: `mnist_classifier::digits::two`,
                            disambiguator: 0,
                        },
                    },
                ),
                Module,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: OnceUseRuleIdx(
                    0,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits::one`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits::one`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `one`,
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
            TokenInfo::UseExpr {
                use_expr_idx: 0,
                rule_idx: OnceUseRuleIdx(
                    10,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                            ),
                            node: ModuleItemSynNode {
                                node_path: ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
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
                                block: Fugitive {
                                    path: FugitivePath(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                    body: Some(
                                        FugitiveBody {
                                            ast_idx_range: ArenaIdxRange(
                                                48..51,
                                            ),
                                        },
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 4,
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
                use_expr_idx: 3,
                rule_idx: OnceUseRuleIdx(
                    11,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits::six`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits::six`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `six`,
                                    token_idx: TokenIdx(
                                        5,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 2,
                rule_idx: OnceUseRuleIdx(
                    19,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                            ),
                            node: ModuleItemSynNode {
                                node_path: ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 52,
                                ident_token: IdentToken {
                                    ident: `is_six`,
                                    token_idx: TokenIdx(
                                        37,
                                    ),
                                },
                                block: Fugitive {
                                    path: FugitivePath(
                                        Id {
                                            value: 35,
                                        },
                                    ),
                                    body: Some(
                                        FugitiveBody {
                                            ast_idx_range: ArenaIdxRange(
                                                26..38,
                                            ),
                                        },
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 7,
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
                use_expr_idx: 6,
                rule_idx: OnceUseRuleIdx(
                    12,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits::zero`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits::zero`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 0,
                                ident_token: IdentToken {
                                    ident: `zero`,
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
            TokenInfo::UseExpr {
                use_expr_idx: 5,
                rule_idx: OnceUseRuleIdx(
                    20,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            ),
                            node: ModuleItemSynNode {
                                node_path: ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
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
                                            value: 27,
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
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 10,
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
                use_expr_idx: 9,
                rule_idx: OnceUseRuleIdx(
                    13,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits::two`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits::two`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 9,
                                ident_token: IdentToken {
                                    ident: `two`,
                                    token_idx: TokenIdx(
                                        19,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 8,
                rule_idx: OnceUseRuleIdx(
                    21,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                            ),
                            node: ModuleItemSynNode {
                                node_path: ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 54,
                                ident_token: IdentToken {
                                    ident: `is_two`,
                                    token_idx: TokenIdx(
                                        113,
                                    ),
                                },
                                block: Fugitive {
                                    path: FugitivePath(
                                        Id {
                                            value: 69,
                                        },
                                    ),
                                    body: Some(
                                        FugitiveBody {
                                            ast_idx_range: ArenaIdxRange(
                                                31..49,
                                            ),
                                        },
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 13,
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
                use_expr_idx: 12,
                rule_idx: OnceUseRuleIdx(
                    14,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits::three`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits::three`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 3,
                                ident_token: IdentToken {
                                    ident: `three`,
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
            TokenInfo::UseExpr {
                use_expr_idx: 11,
                rule_idx: OnceUseRuleIdx(
                    22,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                            ),
                            node: ModuleItemSynNode {
                                node_path: ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 29,
                                ident_token: IdentToken {
                                    ident: `is_three`,
                                    token_idx: TokenIdx(
                                        26,
                                    ),
                                },
                                block: Fugitive {
                                    path: FugitivePath(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                    body: Some(
                                        FugitiveBody {
                                            ast_idx_range: ArenaIdxRange(
                                                1..18,
                                            ),
                                        },
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 16,
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
                use_expr_idx: 15,
                rule_idx: OnceUseRuleIdx(
                    15,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits::five`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits::five`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 5,
                                ident_token: IdentToken {
                                    ident: `five`,
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
            TokenInfo::UseExpr {
                use_expr_idx: 14,
                rule_idx: OnceUseRuleIdx(
                    23,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                            ),
                            node: ModuleItemSynNode {
                                node_path: ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
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
                                block: Fugitive {
                                    path: FugitivePath(
                                        Id {
                                            value: 50,
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
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 19,
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
                use_expr_idx: 18,
                rule_idx: OnceUseRuleIdx(
                    16,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits::seven`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits::seven`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 6,
                                ident_token: IdentToken {
                                    ident: `seven`,
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
            TokenInfo::UseExpr {
                use_expr_idx: 17,
                rule_idx: OnceUseRuleIdx(
                    24,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                            ),
                            node: ModuleItemSynNode {
                                node_path: ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 44,
                                ident_token: IdentToken {
                                    ident: `is_seven`,
                                    token_idx: TokenIdx(
                                        164,
                                    ),
                                },
                                block: Fugitive {
                                    path: FugitivePath(
                                        Id {
                                            value: 56,
                                        },
                                    ),
                                    body: Some(
                                        FugitiveBody {
                                            ast_idx_range: ArenaIdxRange(
                                                28..38,
                                            ),
                                        },
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 22,
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
                use_expr_idx: 21,
                rule_idx: OnceUseRuleIdx(
                    17,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits::eight`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits::eight`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 7,
                                ident_token: IdentToken {
                                    ident: `eight`,
                                    token_idx: TokenIdx(
                                        15,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 20,
                rule_idx: OnceUseRuleIdx(
                    25,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            ),
                            node: ModuleItemSynNode {
                                node_path: ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 19,
                                ident_token: IdentToken {
                                    ident: `is_eight`,
                                    token_idx: TokenIdx(
                                        22,
                                    ),
                                },
                                block: Fugitive {
                                    path: FugitivePath(
                                        Id {
                                            value: 58,
                                        },
                                    ),
                                    body: Some(
                                        FugitiveBody {
                                            ast_idx_range: ArenaIdxRange(
                                                6..13,
                                            ),
                                        },
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 25,
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
                use_expr_idx: 24,
                rule_idx: OnceUseRuleIdx(
                    18,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::digits::nine`,
                            node: SubmoduleSynNode {
                                node_path: SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::digits::nine`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 8,
                                ident_token: IdentToken {
                                    ident: `nine`,
                                    token_idx: TokenIdx(
                                        17,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 23,
                rule_idx: OnceUseRuleIdx(
                    26,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            ),
                            node: ModuleItemSynNode {
                                node_path: ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 38,
                                ident_token: IdentToken {
                                    ident: `is_nine`,
                                    token_idx: TokenIdx(
                                        35,
                                    ),
                                },
                                block: Fugitive {
                                    path: FugitivePath(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                    body: Some(
                                        FugitiveBody {
                                            ast_idx_range: ArenaIdxRange(
                                                17..28,
                                            ),
                                        },
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 27,
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