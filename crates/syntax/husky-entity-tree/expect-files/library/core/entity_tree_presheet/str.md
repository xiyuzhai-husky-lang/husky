Ok(
    EntityTreePresheet {
        module_path: `core::str`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `str`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::str::str`, `Extern`),
                            ),
                            visibility: Visibility::Public,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `str`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `StringLiteral`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::str::StringLiteral`, `Extern`),
                            ),
                            visibility: Visibility::Public,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `StringLiteral`,
                                token_idx: TokenIdx(
                                    6,
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
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)