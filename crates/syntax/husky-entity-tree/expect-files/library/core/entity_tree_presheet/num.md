Ok(
    EntityTreePresheet {
        module_path: `core::num`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "i8",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::i8`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 27,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "i16",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::i16`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 32,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "i32",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::i32`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 37,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "i64",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::i64`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 42,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "u8",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u8`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 47,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "u16",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u16`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 52,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "u32",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u32`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 57,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "u64",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u64`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 62,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "u128",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::u128`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 67,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "usize",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::usize`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 72,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "f32",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::f32`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 77,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "f64",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::num::f64`, `Alien`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 82,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 24,
                    use_expr_idx: 2,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `core::num`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
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
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 41,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
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
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 13,
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
            ],
        },
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)