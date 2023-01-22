Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                            ast_idx: 3,
                            expr_region: ExprRegion(
                                Id {
                                    value: 148,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    8,
                                ),
                            },
                            fields: [
                                RegularStructFieldDecl {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 82,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            10,
                                        ),
                                    },
                                    ty: 1,
                                },
                                RegularStructFieldDecl {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 298,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            14,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                    ty: 6,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        20,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 4,
                            impl_block: ImplBlock(
                                Id {
                                    value: 27,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 149,
                                },
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)