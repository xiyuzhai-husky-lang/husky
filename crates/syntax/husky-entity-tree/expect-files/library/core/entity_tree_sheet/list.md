Ok(
    EntityTreeSheet {
        module_path: `core::list`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `List`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::list::List`, `Extern`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
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
        impl_blocks: [
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `core::list`,
                        ty_path: TypePath(`core::list::List`, `Extern`),
                        disambiguator: 0,
                    },
                    ast_idx: 2,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            8,
                        ),
                    },
                    ty_expr: 42,
                    body: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                0..1,
                            ),
                        },
                    ),
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