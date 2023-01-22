Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ast_idx: 22,
                            expr_region: ExprRegion(
                                Id {
                                    value: 95,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            fields: [
                                RegularStructFieldDecl {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 117,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            8,
                                        ),
                                    },
                                    ty: 4,
                                },
                                RegularStructFieldDecl {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 334,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            16,
                                        ),
                                    },
                                    ty: 8,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        14,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        21,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                            ast_idx: 24,
                            expr_region: ExprRegion(
                                Id {
                                    value: 96,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        147,
                                    ),
                                },
                                parameters: [
                                    RegularParameterDeclPattern {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                156,
                                            ),
                                        },
                                        ty: 11,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            154,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        168,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        169,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                12,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        171,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 23,
                            impl_block: ImplBlock(
                                Id {
                                    value: 29,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        25,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 97,
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 27,
                                    },
                                ),
                                ast_idx: 12,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            28,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 20,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 28,
                                    },
                                ),
                                ast_idx: 13,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 99,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            66,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            68,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 21,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 29,
                                    },
                                ),
                                ast_idx: 14,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 100,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            104,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            106,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)