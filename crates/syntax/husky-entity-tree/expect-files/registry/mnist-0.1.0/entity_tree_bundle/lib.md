Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `mnist`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `MnistLabel`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 10,
                                    ident_token: IdentToken {
                                        ident: `MnistLabel`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BinaryImage28`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 11,
                                    ident_token: IdentToken {
                                        ident: `BinaryImage28`,
                                        token_idx: TokenIdx(
                                            25,
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
        ],
        principal_entity_path_expr_arena: Arena {
            data: [],
        },
    },
)