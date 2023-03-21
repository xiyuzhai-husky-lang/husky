Ok(
    EntityTreePresheet {
        module_path: `core::logic`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `Prop`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::logic::Prop`, `Extern`),
                            ),
                            visibility: Visibility::Public,
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
                    ident: `LogicAnd`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::logic::LogicAnd`, `Structure`),
                            ),
                            visibility: Visibility::Public,
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
                    ident: `LogicOr`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::logic::LogicOr`, `Inductive`),
                            ),
                            visibility: Visibility::Public,
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