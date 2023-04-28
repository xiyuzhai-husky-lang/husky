Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::eight`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `upper_mouth_match`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::eight`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            ast_idx: 18,
                            ident_token: IdentToken {
                                ident: `upper_mouth_match`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_eight`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `big_mouth`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::eight`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            ast_idx: 20,
                            ident_token: IdentToken {
                                ident: `big_mouth`,
                                token_idx: TokenIdx(
                                    85,
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
                    ast_idx: 17,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::eight`,
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