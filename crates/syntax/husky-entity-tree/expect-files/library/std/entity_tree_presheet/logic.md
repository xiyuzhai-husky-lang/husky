Ok(
    EntityTreePresheet {
        module_path: `std::logic`,
        native_symbol_entries: NativeEntitySymbolTable(
            [],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 0,
                    use_expr_idx: 2,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::Public,
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `core`,
                                token_idx: TokenIdx(
                                    2,
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
                            6,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `logic`,
                                token_idx: TokenIdx(
                                    4,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    5,
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
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `core`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    3,
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
            ],
        },
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)