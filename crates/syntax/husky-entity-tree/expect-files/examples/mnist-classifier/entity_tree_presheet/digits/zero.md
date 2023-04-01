Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::zero`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `open_one_match`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            ast_idx: 26,
                            ident_token: IdentToken {
                                ident: `open_one_match`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `almost_closed`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            ast_idx: 27,
                            ident_token: IdentToken {
                                ident: `almost_closed`,
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_zero`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 28,
                            ident_token: IdentToken {
                                ident: `is_zero`,
                                token_idx: TokenIdx(
                                    49,
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