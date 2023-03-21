Ok(
    EntityTreePresheet {
        module_path: `core::raw_bits`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `r32`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::raw_bits::r32`, `Extern`),
                            ),
                            visibility: Visibility::Public,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `r32`,
                                token_idx: TokenIdx(
                                    22,
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