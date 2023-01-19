Ok(
    DeclSheet {
        decls: [
            Err(
                Expr(
                    ExpectRightCurlyBrace(
                        TokenIdx(
                            53,
                        ),
                    ),
                ),
            ),
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ast_idx: 171,
                            expr_page: ExprPage(
                                Id {
                                    value: 165,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    162,
                                ),
                            },
                            fields: [
                                RegularStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 225,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            163,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            164,
                                        ),
                                    },
                                    ty: 1,
                                },
                                RegularStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 270,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            168,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            169,
                                        ),
                                    },
                                    ty: 4,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        167,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        173,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    174,
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
                            path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                            ast_idx: 173,
                            expr_page: ExprPage(
                                Id {
                                    value: 166,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        347,
                                    ),
                                },
                                parameters: [
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                349,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                353,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            351,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        355,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        356,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                2,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        358,
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
                            path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                            ast_idx: 174,
                            expr_page: ExprPage(
                                Id {
                                    value: 167,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        444,
                                    ),
                                },
                                parameters: [
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                446,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                450,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            448,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        452,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        453,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                2,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        455,
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
                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                            ast_idx: 175,
                            expr_page: ExprPage(
                                Id {
                                    value: 168,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        541,
                                    ),
                                },
                                parameters: [
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                543,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                548,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 2,
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                552,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            546,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            550,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        554,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        555,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                4,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        557,
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
                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                            ast_idx: 176,
                            expr_page: ExprPage(
                                Id {
                                    value: 169,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        774,
                                    ),
                                },
                                parameters: [
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                776,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                781,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 2,
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                785,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 3,
                                            variables: ArenaIdxRange(
                                                3..4,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                789,
                                            ),
                                        },
                                        ty: 4,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            779,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            783,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            787,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        791,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        792,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                5,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        794,
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
                            path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                            ast_idx: 177,
                            expr_page: ExprPage(
                                Id {
                                    value: 170,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        1040,
                                    ),
                                },
                                parameters: [
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                1042,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                1047,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            1045,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        1049,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        1050,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                5,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        1054,
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
                            ast_idx: 170,
                            impl_block: ImplBlock(
                                Id {
                                    value: 24,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    81,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        83,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 171,
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 69,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 59,
                                        },
                                    ),
                                ),
                                ast_idx: 3,
                                expr_page: ExprPage(
                                    Id {
                                        value: 172,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            87,
                                        ),
                                    },
                                    parameters: [
                                        ParameterDecl {
                                            pattern: ParameterDeclPattern {
                                                pattern_expr_idx: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            colon: ColonToken {
                                                token_idx: TokenIdx(
                                                    89,
                                                ),
                                            },
                                            ty: 1,
                                        },
                                        ParameterDecl {
                                            pattern: ParameterDeclPattern {
                                                pattern_expr_idx: 1,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            colon: ColonToken {
                                                token_idx: TokenIdx(
                                                    94,
                                                ),
                                            },
                                            ty: 2,
                                        },
                                        ParameterDecl {
                                            pattern: ParameterDeclPattern {
                                                pattern_expr_idx: 2,
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                            },
                                            colon: ColonToken {
                                                token_idx: TokenIdx(
                                                    98,
                                                ),
                                            },
                                            ty: 3,
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                92,
                                            ),
                                        },
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                96,
                                            ),
                                        },
                                    ],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            100,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            101,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    4,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            103,
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
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 70,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                                ast_idx: 4,
                                expr_page: ExprPage(
                                    Id {
                                        value: 173,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            125,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            126,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            127,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            129,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 172,
                            impl_block: ImplBlock(
                                Id {
                                    value: 25,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    175,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        177,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 174,
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
                                            value: 61,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 71,
                                    },
                                ),
                                ast_idx: 21,
                                expr_page: ExprPage(
                                    Id {
                                        value: 175,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            180,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    2,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            184,
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
                                            value: 62,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 72,
                                    },
                                ),
                                ast_idx: 22,
                                expr_page: ExprPage(
                                    Id {
                                        value: 176,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            191,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            193,
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
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 73,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 63,
                                        },
                                    ),
                                ),
                                ast_idx: 23,
                                expr_page: ExprPage(
                                    Id {
                                        value: 177,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            316,
                                        ),
                                    },
                                    parameters: [
                                        ParameterDecl {
                                            pattern: ParameterDeclPattern {
                                                pattern_expr_idx: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            colon: ColonToken {
                                                token_idx: TokenIdx(
                                                    318,
                                                ),
                                            },
                                            ty: 1,
                                        },
                                        ParameterDecl {
                                            pattern: ParameterDeclPattern {
                                                pattern_expr_idx: 1,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            colon: ColonToken {
                                                token_idx: TokenIdx(
                                                    323,
                                                ),
                                            },
                                            ty: 2,
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                321,
                                            ),
                                        },
                                    ],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            325,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            326,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    3,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            328,
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