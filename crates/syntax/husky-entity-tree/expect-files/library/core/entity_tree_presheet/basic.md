Ok(
    EntityTreePresheet {
        module_path: `core::basic`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `bool`,
                    visibility: Visibility::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::basic::bool`, `Extern`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `bool`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `never`,
                    visibility: Visibility::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::basic::never`, `Extern`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `never`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `unit`,
                    visibility: Visibility::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::basic::unit`, `Extern`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `unit`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Trait`,
                    visibility: Visibility::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::basic::Trait`, `Structure`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Trait`,
                                token_idx: TokenIdx(
                                    14,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Module`,
                    visibility: Visibility::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::basic::Module`, `Structure`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `Module`,
                                token_idx: TokenIdx(
                                    18,
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
            data: [],
        },
        errors: [],
    },
)