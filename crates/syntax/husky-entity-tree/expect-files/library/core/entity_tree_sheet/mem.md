Ok(
    EntityTreeSheet {
        module_path: `core::mem`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Ref`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::mem::Ref`, `Extern`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Ref`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RefMut`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::mem::RefMut`, `Extern`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `RefMut`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Leash`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::mem::Leash`, `Extern`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Leash`,
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                        },
                    ),
                },
            ],
        ),
        impl_blocks: [
            ImplBlock::TraitForType(
                TraitForTypeImplBlock {
                    id: TraitForTypeImplBlockId {
                        module_path: `core::mem`,
                        trai_path: TraitPath(`core::marker::Copy`),
                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                        disambiguator: 0,
                    },
                    ast_idx: 3,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            30,
                        ),
                    },
                    trai_expr: 1,
                    for_token: TokenIdx(
                        35,
                    ),
                    ty_expr: 2,
                    items: None,
                },
            ),
        ],
        use_expr_rules: UseExprRules(
            [],
        ),
        use_all_rules: UseAllRules(
            [],
        ),
        errors: [],
    },
)