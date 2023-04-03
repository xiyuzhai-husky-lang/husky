Ok(
    EntityTreeSheet {
        module_path: `core::logic`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Prop`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::logic::Prop`, `Extern`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Prop`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LogicAnd`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::logic::LogicAnd`, `Structure`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `LogicAnd`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LogicOr`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::logic::LogicOr`, `Inductive`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 4,
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