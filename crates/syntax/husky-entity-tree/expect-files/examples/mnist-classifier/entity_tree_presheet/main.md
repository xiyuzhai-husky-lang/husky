Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `connected_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 11,
                            ident_token: IdentToken {
                                ident: `connected_component`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `raw_contour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 12,
                            ident_token: IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `geom2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 13,
                            ident_token: IdentToken {
                                ident: `geom2d`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `line_segment_sketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 14,
                            ident_token: IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `fermi`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::fermi`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 15,
                            ident_token: IdentToken {
                                ident: `fermi`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `digits`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 16,
                            ident_token: IdentToken {
                                ident: `digits`,
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `major`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::major`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 17,
                            ident_token: IdentToken {
                                ident: `major`,
                                token_idx: TokenIdx(
                                    13,
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
                    ast_idx: 23,
                    use_expr_idx: 6,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    46,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            5..6,
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
                UseExpr::Err(
                    UseExprError::Original(
                        OriginalUseExprError::InvalidSelfAsRoot {
                            use_token: UseToken {
                                token_idx: TokenIdx(
                                    14,
                                ),
                            },
                            self_token: SelfValueToken {
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        },
                    ),
                ),
                UseExpr::Err(
                    UseExprError::Original(
                        OriginalUseExprError::InvalidSelfAsRoot {
                            use_token: UseToken {
                                token_idx: TokenIdx(
                                    20,
                                ),
                            },
                            self_token: SelfValueToken {
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                        },
                    ),
                ),
                UseExpr::Err(
                    UseExprError::Original(
                        OriginalUseExprError::InvalidSelfAsRoot {
                            use_token: UseToken {
                                token_idx: TokenIdx(
                                    26,
                                ),
                            },
                            self_token: SelfValueToken {
                                token_idx: TokenIdx(
                                    27,
                                ),
                            },
                        },
                    ),
                ),
                UseExpr::Err(
                    UseExprError::Original(
                        OriginalUseExprError::InvalidSelfAsRoot {
                            use_token: UseToken {
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                            self_token: SelfValueToken {
                                token_idx: TokenIdx(
                                    34,
                                ),
                            },
                        },
                    ),
                ),
                UseExpr::Err(
                    UseExprError::Original(
                        OriginalUseExprError::InvalidSelfAsRoot {
                            use_token: UseToken {
                                token_idx: TokenIdx(
                                    39,
                                ),
                            },
                            self_token: SelfValueToken {
                                token_idx: TokenIdx(
                                    40,
                                ),
                            },
                        },
                    ),
                ),
                UseExpr::All {
                    star_token: StarToken(
                        TokenIdx(
                            48,
                        ),
                    ),
                },
                UseExpr::Parent(
                    ParentUseExpr {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    46,
                                ),
                            },
                        ),
                        scope_resolution_token: Ok(
                            ScopeResolutionToken(
                                TokenIdx(
                                    47,
                                ),
                            ),
                        ),
                        children: Ok(
                            UseExprChildren::Single {
                                child: 5,
                            },
                        ),
                    },
                ),
            ],
        },
        errors: [],
    },
)