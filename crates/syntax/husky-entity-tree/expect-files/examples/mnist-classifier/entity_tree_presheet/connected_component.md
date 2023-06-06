Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::connected_component`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `ConnectedComponentDistribution`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 120,
                            ident_token: IdentToken {
                                ident: `ConnectedComponentDistribution`,
                                token_idx: TokenIdx(
                                    12,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `EffHoles`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 121,
                            ident_token: IdentToken {
                                ident: `EffHoles`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `hole_tmpl`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 122,
                            ident_token: IdentToken {
                                ident: `hole_tmpl`,
                                token_idx: TokenIdx(
                                    45,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `ConnectedComponent`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 123,
                            ident_token: IdentToken {
                                ident: `ConnectedComponent`,
                                token_idx: TokenIdx(
                                    71,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `horizontal_extend`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 126,
                            ident_token: IdentToken {
                                ident: `horizontal_extend`,
                                token_idx: TokenIdx(
                                    568,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `find_connected_components`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 127,
                            ident_token: IdentToken {
                                ident: `find_connected_components`,
                                token_idx: TokenIdx(
                                    654,
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
                    ast_idx: 118,
                    use_expr_idx: 2,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    1,
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
                    ast_idx: 119,
                    use_expr_idx: 4,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            3..4,
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
                            5,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `raw_contour`,
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
                                child: 0,
                            },
                        ),
                    },
                ),
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
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
                                child: 1,
                            },
                        ),
                    },
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            9,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    8,
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
            ],
        },
        errors: [],
    },
)