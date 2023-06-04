Ok(
    EntityTreePresheet {
        module_path: `core::result`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `Result`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `Result`,
                                token_idx: TokenIdx(
                                    2,
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