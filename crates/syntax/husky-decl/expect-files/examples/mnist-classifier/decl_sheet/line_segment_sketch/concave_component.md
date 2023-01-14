Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    PropsStruct(
                        PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ast_idx: 76,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 141,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    31,
                                ),
                            },
                            fields: [
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 83,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            32,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                    ty: 1,
                                },
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 270,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            37,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            38,
                                        ),
                                    },
                                    ty: 6,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        36,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        43,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    44,
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
                            path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                            ast_idx: 78,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 142,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        512,
                                    ),
                                },
                                decls: [
                                    ParameterDecl {
                                        pattern: ParameterPattern {
                                            pattern_expr_idx: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                514,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        517,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        518,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                4,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        522,
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
                            ast_idx: 77,
                            impl_block: ImplBlock(
                                Id {
                                    value: 26,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    45,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        47,
                                    ),
                                },
                            ),
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 143,
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
                                            value: 45,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 55,
                                    },
                                ),
                                ast_idx: 42,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 144,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            50,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            52,
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
                                            value: 46,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 56,
                                    },
                                ),
                                ast_idx: 43,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 145,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            58,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            60,
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
                                            value: 47,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 57,
                                    },
                                ),
                                ast_idx: 44,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 146,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            76,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            78,
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
                                            value: 48,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 58,
                                    },
                                ),
                                ast_idx: 45,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 147,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            163,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            165,
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
                                            value: 49,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 59,
                                    },
                                ),
                                ast_idx: 46,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 148,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            234,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            236,
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
                                            value: 50,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 60,
                                    },
                                ),
                                ast_idx: 47,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 149,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            363,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            365,
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
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 61,
                                    },
                                ),
                                ast_idx: 48,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 150,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            380,
                                        ),
                                    },
                                    decls: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            381,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            382,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            384,
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
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 52,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 62,
                                    },
                                ),
                                ast_idx: 49,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 151,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            417,
                                        ),
                                    },
                                    decls: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            418,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            419,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            421,
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
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 53,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 63,
                                    },
                                ),
                                ast_idx: 50,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 152,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            437,
                                        ),
                                    },
                                    decls: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            438,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            439,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            441,
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
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 54,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 64,
                                    },
                                ),
                                ast_idx: 51,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 153,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            457,
                                        ),
                                    },
                                    decls: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            458,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            459,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            461,
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
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 55,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 65,
                                    },
                                ),
                                ast_idx: 52,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 154,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            473,
                                        ),
                                    },
                                    decls: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            474,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            475,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            477,
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
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 56,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 66,
                                    },
                                ),
                                ast_idx: 53,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 155,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            491,
                                        ),
                                    },
                                    decls: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            492,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            493,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            495,
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