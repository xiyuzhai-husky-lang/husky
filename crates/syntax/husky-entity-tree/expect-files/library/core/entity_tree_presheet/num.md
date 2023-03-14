Ok(
    EntityTreePresheet {
        module_path: `core::num`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "i8",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::i8`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 31,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "i16",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::i16`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 36,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "i32",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 41,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "i64",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::i64`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 46,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "i128",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::i128`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 51,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "isize",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 56,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "u8",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u8`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 61,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "u16",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u16`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 66,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "u32",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u32`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 71,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "u64",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u64`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 76,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "u128",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u128`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 81,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "usize",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 86,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "f32",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 91,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "f64",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::f64`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 96,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 28,
                    use_expr_idx: 2,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `core::num`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: ParentNameToken::Ident(
                            IdentToken {
                                ident: `core`,
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
            ],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 44,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                },
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 0,
                            },
                        ),
                    },
                ),
                Parent(
                    ParentUseExpr {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                ),
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
                            Single {
                                child: 1,
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