Ok(
    EntityTreeSheet {
        module_path: `core::result`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Result`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
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
        impl_blocks: [
            ImplBlock::TraitForType(
                TraitForTypeImplBlock {
                    id: TraitForTypeImplBlockId {
                        module_path: `core::result`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_path: TypePath(`core::result::Result`, `Enum`),
                        disambiguator: 0,
                    },
                    ast_idx: 6,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            18,
                        ),
                    },
                    trai_expr: 47,
                    for_token: TokenIdx(
                        36,
                    ),
                    ty_expr: 48,
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    3..5,
                                ),
                            },
                        ),
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