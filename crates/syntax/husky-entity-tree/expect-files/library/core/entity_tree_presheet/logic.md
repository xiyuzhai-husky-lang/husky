Ok(
    EntityTreePresheet {
        module_path: `core::logic`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "Prop",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::logic::Prop`, `Extern`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Prop`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "LogicAnd",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::logic::LogicAnd`, `Structure`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `LogicAnd`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "LogicOr",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::logic::LogicOr`, `Inductive`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `LogicOr`,
                                token_idx: TokenIdx(
                                    28,
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