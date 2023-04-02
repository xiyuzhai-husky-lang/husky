Ok(
    EntityTreeSheet {
        module_path: `core::clone`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Clone`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                            visibility: Visibility::Pub,
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