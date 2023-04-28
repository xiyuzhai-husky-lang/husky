Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::major`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `connected_components`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 19,
                            ident_token: IdentToken {
                                ident: `connected_components`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `major_connected_component`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 20,
                            ident_token: IdentToken {
                                ident: `major_connected_component`,
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `ignored_connected_components_row_span_sum_sum`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 21,
                            ident_token: IdentToken {
                                ident: `ignored_connected_components_row_span_sum_sum`,
                                token_idx: TokenIdx(
                                    70,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `major_raw_contours`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 22,
                            ident_token: IdentToken {
                                ident: `major_raw_contours`,
                                token_idx: TokenIdx(
                                    104,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `major_raw_contour`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 23,
                            ident_token: IdentToken {
                                ident: `major_raw_contour`,
                                token_idx: TokenIdx(
                                    116,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `major_line_segment_sketch`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 24,
                            ident_token: IdentToken {
                                ident: `major_line_segment_sketch`,
                                token_idx: TokenIdx(
                                    129,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `major_concave_components`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 25,
                            ident_token: IdentToken {
                                ident: `major_concave_components`,
                                token_idx: TokenIdx(
                                    139,
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
                    ast_idx: 18,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::major`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
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
                        parent_name_token: NameToken::Crate(
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