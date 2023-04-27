Ok(
    EntityTreePresheet {
        module_path: `core::list`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `List`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::list::List`, `Extern`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `List`,
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