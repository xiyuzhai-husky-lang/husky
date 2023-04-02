Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::four`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `left_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::left_components`, `Val`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 38,
                            ident_token: IdentToken {
                                ident: `left_components`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `left_coordinate_max`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 39,
                            ident_token: IdentToken {
                                ident: `left_coordinate_max`,
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `components_max_downwards`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 40,
                            ident_token: IdentToken {
                                ident: `components_max_downwards`,
                                token_idx: TokenIdx(
                                    40,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `components_max_heights`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 41,
                            ident_token: IdentToken {
                                ident: `components_max_heights`,
                                token_idx: TokenIdx(
                                    54,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_four`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::is_four`, `Val`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 42,
                            ident_token: IdentToken {
                                ident: `is_four`,
                                token_idx: TokenIdx(
                                    68,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `displacement_downwards`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 43,
                            ident_token: IdentToken {
                                ident: `displacement_downwards`,
                                token_idx: TokenIdx(
                                    251,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `cc_box_heights`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::four`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            ast_idx: 44,
                            ident_token: IdentToken {
                                ident: `cc_box_heights`,
                                token_idx: TokenIdx(
                                    280,
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
                    ast_idx: 37,
                    use_expr_idx: 1,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::four`,
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