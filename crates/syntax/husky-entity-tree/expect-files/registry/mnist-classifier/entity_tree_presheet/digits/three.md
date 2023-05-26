Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::three`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `three_fermi_match`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::three`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            ast_idx: 28,
                            ident_token: IdentToken {
                                ident: `three_fermi_match`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_three`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
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
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `uparc`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::three`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            ast_idx: 30,
                            ident_token: IdentToken {
                                ident: `uparc`,
                                token_idx: TokenIdx(
                                    156,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `downarc`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::three`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            ast_idx: 31,
                            ident_token: IdentToken {
                                ident: `downarc`,
                                token_idx: TokenIdx(
                                    190,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `back`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::three`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            ast_idx: 32,
                            ident_token: IdentToken {
                                ident: `back`,
                                token_idx: TokenIdx(
                                    224,
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
                    ast_idx: 27,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::three`,
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