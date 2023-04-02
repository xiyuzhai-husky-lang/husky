Ok(
    EntityTreeSheet {
        module_path: `core::list`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `List`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::list::List`, `Extern`),
                            ),
                            visibility: Visibility::Pub,
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
            ImplBlock::IllFormed(
                IllFormedImplBlock {
                    id: IllFormedImplBlockId {
                        module: `core::list`,
                        disambiguator: 0,
                    },
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            8,
                        ),
                    },
                    ast_idx: 2,
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    ill_form: ImplBlockIllForm::MajorPath(
                        MajorPathExprError::Original(
                            OriginalMajorPathExprError::ExpectIdent(
                                TokenIdx(
                                    15,
                                ),
                            ),
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