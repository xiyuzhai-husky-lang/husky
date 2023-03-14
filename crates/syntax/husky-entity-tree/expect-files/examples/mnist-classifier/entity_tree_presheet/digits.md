Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "zero",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::zero`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 0,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "one",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::one`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 1,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "six",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::six`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 2,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "three",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::three`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 3,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "four",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::four`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 4,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "five",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::five`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 5,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "six",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::six`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 6,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "seven",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::seven`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 7,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "eight",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::eight`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 8,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "nine",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::nine`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 9,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 10,
                    use_expr_idx: 1,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::Public,
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `one`,
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 11,
                    use_expr_idx: 3,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `six`,
                                token_idx: TokenIdx(
                                    26,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            2..3,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 12,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `zero`,
                                token_idx: TokenIdx(
                                    30,
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
                    ast_idx: 13,
                    use_expr_idx: 7,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `two`,
                                token_idx: TokenIdx(
                                    34,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            6..7,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 14,
                    use_expr_idx: 9,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `three`,
                                token_idx: TokenIdx(
                                    38,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            8..9,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 15,
                    use_expr_idx: 11,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `five`,
                                token_idx: TokenIdx(
                                    42,
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
                    ast_idx: 16,
                    use_expr_idx: 13,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `six`,
                                token_idx: TokenIdx(
                                    46,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            12..13,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 17,
                    use_expr_idx: 15,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `seven`,
                                token_idx: TokenIdx(
                                    50,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            14..15,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 18,
                    use_expr_idx: 17,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `eight`,
                                token_idx: TokenIdx(
                                    54,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            16..17,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 19,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `nine`,
                                token_idx: TokenIdx(
                                    58,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            18..19,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_one`,
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `one`,
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    23,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_six`,
                        token_idx: TokenIdx(
                            28,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `six`,
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
                                child: 2,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_zero`,
                        token_idx: TokenIdx(
                            32,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `zero`,
                                token_idx: TokenIdx(
                                    30,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    31,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_two`,
                        token_idx: TokenIdx(
                            36,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `two`,
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
                                child: 6,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_three`,
                        token_idx: TokenIdx(
                            40,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `three`,
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
                                child: 8,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_five`,
                        token_idx: TokenIdx(
                            44,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `five`,
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
                                child: 10,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_six`,
                        token_idx: TokenIdx(
                            48,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `six`,
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
                                child: 12,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_seven`,
                        token_idx: TokenIdx(
                            52,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `seven`,
                                token_idx: TokenIdx(
                                    50,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    51,
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
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_eight`,
                        token_idx: TokenIdx(
                            56,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `eight`,
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
                                child: 16,
                            },
                        ),
                    },
                ),
                UseExpr::Leaf {
                    ident_token: IdentToken {
                        ident: `is_nine`,
                        token_idx: TokenIdx(
                            60,
                        ),
                    },
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `nine`,
                                token_idx: TokenIdx(
                                    58,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    59,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 18,
                            },
                        ),
                    },
                ),
            ],
        },
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)