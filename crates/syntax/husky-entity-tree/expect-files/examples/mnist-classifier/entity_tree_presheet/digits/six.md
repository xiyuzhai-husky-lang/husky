Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::six`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `six_match`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::six`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            ast_idx: 50,
                            ident_token: IdentToken {
                                ident: `six_match`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `six_match_refined1`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::six`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            ast_idx: 51,
                            ident_token: IdentToken {
                                ident: `six_match_refined1`,
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_six`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 52,
                            ident_token: IdentToken {
                                ident: `is_six`,
                                token_idx: TokenIdx(
                                    39,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `upmost`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::six`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            ast_idx: 53,
                            ident_token: IdentToken {
                                ident: `upmost`,
                                token_idx: TokenIdx(
                                    305,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `bottom1`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::six`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            ast_idx: 54,
                            ident_token: IdentToken {
                                ident: `bottom1`,
                                token_idx: TokenIdx(
                                    334,
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
                    ast_idx: 49,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::six`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Super(
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
                            3,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Super(
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