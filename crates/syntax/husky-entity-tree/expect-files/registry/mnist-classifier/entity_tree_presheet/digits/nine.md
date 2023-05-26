Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::nine`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `nine_match`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::nine`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            ast_idx: 36,
                            ident_token: IdentToken {
                                ident: `nine_match`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `nine_match_refine`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::nine`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            ast_idx: 37,
                            ident_token: IdentToken {
                                ident: `nine_match_refine`,
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_nine`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
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
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `downmost`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::nine`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            ast_idx: 39,
                            ident_token: IdentToken {
                                ident: `downmost`,
                                token_idx: TokenIdx(
                                    208,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `big_cc`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::nine`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            ast_idx: 40,
                            ident_token: IdentToken {
                                ident: `big_cc`,
                                token_idx: TokenIdx(
                                    237,
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
                    ast_idx: 35,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::nine`,
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