Ok(
    EntitySynTreePresheet {
        module_path: `core::prelude`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [],
        },
        use_one_trackers: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 1,
                    use_expr_idx: 3,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            2..3,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 2,
                    use_expr_idx: 6,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            5..6,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 3,
                    use_expr_idx: 9,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    17,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            8..9,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 4,
                    use_expr_idx: 12,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            11..12,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 5,
                    use_expr_idx: 15,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    31,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            14..15,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 6,
                    use_expr_idx: 18,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    38,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            17..18,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 7,
                    use_expr_idx: 21,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    45,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            20..21,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 8,
                    use_expr_idx: 24,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    52,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            23..24,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 9,
                    use_expr_idx: 27,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    59,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            26..27,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 10,
                    use_expr_idx: 30,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    66,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            29..30,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Unresolved,
                },
                OnceUseRule {
                    ast_idx: 11,
                    use_expr_idx: 33,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    73,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            32..33,
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
                            7,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `basic`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    6,
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
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    4,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 2,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            14,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `num`,
                                token_idx: TokenIdx(
                                    12,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    13,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 4,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    11,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 5,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            21,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `raw_bits`,
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    20,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 7,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    17,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    18,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 8,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            28,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `fmt`,
                                token_idx: TokenIdx(
                                    26,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    27,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 10,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    25,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 11,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            35,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `visual`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    34,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 13,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    31,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    32,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 14,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            42,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `marker`,
                                token_idx: TokenIdx(
                                    40,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    41,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 16,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    38,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    39,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 17,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            49,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `clone`,
                                token_idx: TokenIdx(
                                    47,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    48,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 19,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    45,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    46,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 20,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            56,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `option`,
                                token_idx: TokenIdx(
                                    54,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    55,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 22,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    52,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    53,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 23,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            63,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `cmp`,
                                token_idx: TokenIdx(
                                    61,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    62,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 25,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    59,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    60,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 26,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            70,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `slice`,
                                token_idx: TokenIdx(
                                    68,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    69,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 28,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    66,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    67,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 29,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            77,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `vec`,
                                token_idx: TokenIdx(
                                    75,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    76,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 31,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    73,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    74,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 32,
                            },
                        ),
                    },
                ),
            ],
        },
        errors: [],
    },
)