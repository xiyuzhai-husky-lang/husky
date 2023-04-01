Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::eight`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `upper_mouth_match`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::eight`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Feature`),
                            ),
                            visibility: Visibility::PubUnder(
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
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                            ),
                            visibility: Visibility::PubUnder(
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
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::eight`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
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