Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    PropsStruct(
                        PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                            ast_idx: 120,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 45,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    16,
                                ),
                            },
                            fields: [
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 114,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            18,
                                        ),
                                    },
                                    ty: 0,
                                },
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 115,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            21,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            22,
                                        ),
                                    },
                                    ty: 1,
                                },
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 116,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            26,
                                        ),
                                    },
                                    ty: 2,
                                },
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 117,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            29,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                    ty: 3,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        20,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        28,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        32,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    PropsStruct(
                        PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                            ast_idx: 121,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 46,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    36,
                                ),
                            },
                            fields: [
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 119,
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
                                    ty: 4,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        44,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    45,
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
                            path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                            ast_idx: 122,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 47,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        48,
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
                                                50,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        53,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        54,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                3,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        57,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    PropsStruct(
                        PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ast_idx: 123,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 48,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    73,
                                ),
                            },
                            fields: [
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 125,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            74,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            75,
                                        ),
                                    },
                                    ty: 0,
                                },
                            ],
                            separators: [],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    77,
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
                            path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                            ast_idx: 125,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 49,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        537,
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
                                                539,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterPattern {
                                            pattern_expr_idx: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                543,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            541,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        545,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        546,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                2,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        548,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                            ast_idx: 126,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 50,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        622,
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
                                                624,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        626,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        627,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                3,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        631,
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
                            ast_idx: 124,
                            impl_block: ImplBlock(
                                Id {
                                    value: 17,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    78,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        80,
                                    ),
                                },
                            ),
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 51,
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
                                            value: 9,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 17,
                                    },
                                ),
                                ast_idx: 67,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 52,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            83,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    2,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            87,
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
                                            value: 10,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 18,
                                    },
                                ),
                                ast_idx: 68,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 53,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            94,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            96,
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
                                            value: 11,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 19,
                                    },
                                ),
                                ast_idx: 69,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            155,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            157,
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
                                            value: 12,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 20,
                                    },
                                ),
                                ast_idx: 70,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 55,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            207,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            209,
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
                                            value: 13,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 21,
                                    },
                                ),
                                ast_idx: 71,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 56,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            243,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            245,
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
                                            value: 14,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 22,
                                    },
                                ),
                                ast_idx: 72,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 57,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            274,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            276,
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
                                            value: 15,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 23,
                                    },
                                ),
                                ast_idx: 73,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 58,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            388,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            390,
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
                                            value: 16,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 24,
                                    },
                                ),
                                ast_idx: 74,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 59,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            400,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            402,
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
                                            value: 17,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 25,
                                    },
                                ),
                                ast_idx: 75,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 60,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            412,
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
                                                    414,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            416,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            417,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            419,
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
                                            value: 18,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 26,
                                    },
                                ),
                                ast_idx: 76,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 61,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            471,
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
                                                    473,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            475,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            476,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            478,
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