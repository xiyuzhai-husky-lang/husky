Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::two`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `two_match`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::two_match`, `Var`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 50,
                            ident_token: IdentToken {
                                ident: `two_match`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `left_cc_pattern`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 51,
                            ident_token: IdentToken {
                                ident: `left_cc_pattern`,
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `right_cc_pattern`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 52,
                            ident_token: IdentToken {
                                ident: `right_cc_pattern`,
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `down_cc_pattern`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 53,
                            ident_token: IdentToken {
                                ident: `down_cc_pattern`,
                                token_idx: TokenIdx(
                                    80,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_two`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::is_two`, `Var`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 54,
                            ident_token: IdentToken {
                                ident: `is_two`,
                                token_idx: TokenIdx(
                                    113,
                                ),
                            },
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [
                UseExpr::Err(
                    UseExprError::Original(
                        OriginalUseExprError::ExpectUseExpr(
                            TokenIdx(
                                1,
                            ),
                        ),
                    ),
                ),
            ],
        },
        errors: [],
    },
)