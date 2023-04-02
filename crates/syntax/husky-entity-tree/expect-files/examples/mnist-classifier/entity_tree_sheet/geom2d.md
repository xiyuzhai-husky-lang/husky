Ok(
    EntityTreeSheet {
        module_path: `mnist_classifier::geom2d`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Point2d`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 78,
                            ident_token: IdentToken {
                                ident: `Point2d`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativePoint2d`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 80,
                            ident_token: IdentToken {
                                ident: `RelativePoint2d`,
                                token_idx: TokenIdx(
                                    144,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Vector2d`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 81,
                            ident_token: IdentToken {
                                ident: `Vector2d`,
                                token_idx: TokenIdx(
                                    157,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ClosedRange`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 83,
                            ident_token: IdentToken {
                                ident: `ClosedRange`,
                                token_idx: TokenIdx(
                                    488,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BoundingBox`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 85,
                            ident_token: IdentToken {
                                ident: `BoundingBox`,
                                token_idx: TokenIdx(
                                    596,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativeBoundingBox`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 87,
                            ident_token: IdentToken {
                                ident: `RelativeBoundingBox`,
                                token_idx: TokenIdx(
                                    732,
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 79,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            13,
                        ),
                    },
                    ty_expr: 2,
                    body: ArenaIdxRange(
                        5..10,
                    ),
                },
            ),
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 82,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            168,
                        ),
                    },
                    ty_expr: 3,
                    body: ArenaIdxRange(
                        41..49,
                    ),
                },
            ),
            ImplBlock::IllFormed(
                IllFormedImplBlock {
                    id: IllFormedImplBlockId {
                        module: `mnist_classifier::geom2d`,
                        disambiguator: 0,
                    },
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            499,
                        ),
                    },
                    ast_idx: 84,
                    body: ArenaIdxRange(
                        56..58,
                    ),
                    ill_form: ImplBlockIllForm::MajorPath(
                        MajorPathExprError::Original(
                            OriginalMajorPathExprError::UnrecognizedIdent(
                                IdentToken {
                                    ident: `CloseRange`,
                                    token_idx: TokenIdx(
                                        500,
                                    ),
                                },
                            ),
                        ),
                    ),
                },
            ),
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 86,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            609,
                        ),
                    },
                    ty_expr: 4,
                    body: ArenaIdxRange(
                        64..70,
                    ),
                },
            ),
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 88,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            743,
                        ),
                    },
                    ty_expr: 5,
                    body: ArenaIdxRange(
                        74..78,
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