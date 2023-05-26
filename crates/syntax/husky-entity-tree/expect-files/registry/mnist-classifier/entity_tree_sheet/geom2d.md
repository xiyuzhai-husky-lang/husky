Ok(
    EntityTreeSheet {
        module_path: `mnist_classifier::geom2d`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Point2d`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                            visibility: Scope::Pub,
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
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                            visibility: Scope::Pub,
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
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                            visibility: Scope::Pub,
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
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                            visibility: Scope::Pub,
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
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                            visibility: Scope::Pub,
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
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                            visibility: Scope::Pub,
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
                    ty_expr: 6,
                    body: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                5..10,
                            ),
                        },
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
                    ty_expr: 7,
                    body: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                41..49,
                            ),
                        },
                    ),
                },
            ),
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 84,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            499,
                        ),
                    },
                    ty_expr: 8,
                    body: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                56..58,
                            ),
                        },
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
                    ty_expr: 9,
                    body: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                64..70,
                            ),
                        },
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
                    ty_expr: 10,
                    body: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                74..78,
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