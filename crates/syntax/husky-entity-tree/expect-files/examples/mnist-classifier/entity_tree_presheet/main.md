Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `connected_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::connected_component`,
                            visibility: Visibility::PubUnder(
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
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `raw_contour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::raw_contour`,
                            visibility: Visibility::PubUnder(
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
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `geom2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 13,
                            ident_token: IdentToken {
                                ident: `geom2d`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `line_segment_sketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch`,
                            visibility: Visibility::PubUnder(
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
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `fermi`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::fermi`,
                            visibility: Visibility::PubUnder(
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
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `digits`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits`,
                            visibility: Visibility::PubUnder(
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
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `major`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::major`,
                            visibility: Visibility::PubUnder(
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
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 18,
                    use_expr_idx: 2,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            1..2,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 5,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            4..5,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 20,
                    use_expr_idx: 8,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    27,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            7..8,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 21,
                    use_expr_idx: 11,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    34,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            10..11,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 22,
                    use_expr_idx: 14,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    40,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            13..14,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 23,
                    use_expr_idx: 16,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    46,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            15..16,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
            ],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            19,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `major`,
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
                                child: 0,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    16,
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
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            25,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `digits`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    24,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 3,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    22,
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
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            31,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `fermi`,
                                token_idx: TokenIdx(
                                    29,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    30,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 6,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    27,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    28,
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
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            38,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    36,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    37,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 9,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    34,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    35,
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
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            44,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    42,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    43,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 12,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
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
                                child: 13,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            48,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    46,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    47,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 15,
                            },
                        ),
                    },
                ),
            ],
        },
        errors: [],
    },
)