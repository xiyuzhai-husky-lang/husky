Ok(
    EntityTreePresheet {
        module_path: `core::mem`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "Ref",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::mem::Ref`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Ref`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "RefMut",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::mem::RefMut`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `RefMut`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "Leash",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::mem::Leash`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Leash`,
                                token_idx: TokenIdx(
                                    24,
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