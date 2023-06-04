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
            ImplBlock::IllFormed(
                IllFormedImplBlock {
                    id: IllFormedImplBlockId {
                        module: `core::result`,
                        disambiguator: 0,
                    },
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            18,
                        ),
                    },
                    ast_idx: 6,
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    3..5,
                                ),
                            },
                        ),
                    ),
                    ill_form: ImplBlockIllForm::MajorPath(
                        MajorPathExprError::Original(
                            OriginalMajorPathExprError::ExpectedIdent(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        26,
                                    ),
                                    drained: false,
                                },
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