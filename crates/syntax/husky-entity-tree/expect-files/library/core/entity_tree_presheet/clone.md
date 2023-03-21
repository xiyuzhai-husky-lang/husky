Ok(
    EntityTreePresheet {
        module_path: `core::clone`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `Clone`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                            visibility: Visibility::Public,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `Clone`,
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
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)