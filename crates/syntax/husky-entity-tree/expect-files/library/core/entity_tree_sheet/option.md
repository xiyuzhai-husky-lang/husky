Ok(
    EntityTreeSheet {
        module_path: `core::option`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Option`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Option`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
            ],
        ),
        impl_blocks: [],
        use_expr_rules: UseExprRules(
            [],
        ),
        use_all_rules: UseAllRules(
            [],
        ),
        errors: [],
    },
)