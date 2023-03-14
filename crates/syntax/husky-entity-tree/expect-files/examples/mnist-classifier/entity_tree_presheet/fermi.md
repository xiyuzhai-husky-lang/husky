Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::fermi`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "FermiMatchResult",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 22,
                            ident_token: IdentToken {
                                ident: `FermiMatchResult`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "fermi_match",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 24,
                            ident_token: IdentToken {
                                ident: `fermi_match`,
                                token_idx: TokenIdx(
                                    146,
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
                    ast_idx: 21,
                    use_expr_idx: 1,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::fermi`,
                        ),
                    },
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
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)