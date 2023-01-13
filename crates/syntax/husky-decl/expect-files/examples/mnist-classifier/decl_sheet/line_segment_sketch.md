Ok(
    DeclSheet {
        decls: [
            Err(
                Expr(
                    ExpectRightCurlyBrace(
                        TokenIdx(
                            45,
                        ),
                    ),
                ),
            ),
            Ok(
                Type(
                    PropsStruct(
                        PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ast_idx: 170,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 163,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    149,
                                ),
                            },
                            fields: [
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 228,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            150,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            151,
                                        ),
                                    },
                                    ty: 1,
                                },
                                PropsStructFieldDecl {
                                    ident: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 275,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            155,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            156,
                                        ),
                                    },
                                    ty: 4,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        154,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        160,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    161,
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
                            ast_idx: 172,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 164,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        328,
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
                                                330,
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
                                                334,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            332,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        336,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        337,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                2,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        339,
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
                            ast_idx: 173,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 165,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        425,
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
                                                427,
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
                                                431,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            429,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        433,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        434,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                2,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        436,
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
                            ast_idx: 174,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 166,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        519,
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
                                                521,
                                            ),
                                        },
                                        ty: 1,
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
                                                526,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterPattern {
                                            pattern_expr_idx: 2,
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                530,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            524,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            528,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        532,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        533,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                4,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        535,
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
                            ast_idx: 175,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 167,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        752,
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
                                                754,
                                            ),
                                        },
                                        ty: 1,
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
                                                759,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterPattern {
                                            pattern_expr_idx: 2,
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                763,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                    ParameterDecl {
                                        pattern: ParameterPattern {
                                            pattern_expr_idx: 3,
                                            variables: ArenaIdxRange(
                                                3..4,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                767,
                                            ),
                                        },
                                        ty: 4,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            757,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            761,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            765,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        769,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        770,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                5,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        772,
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
                            ast_idx: 176,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 168,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        1018,
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
                                                1020,
                                            ),
                                        },
                                        ty: 1,
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
                                                1025,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            1023,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        1027,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        1028,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                5,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        1032,
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
                            ast_idx: 169,
                            impl_block: ImplBlock(
                                Id {
                                    value: 24,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    73,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        75,
                                    ),
                                },
                            ),
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 169,
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
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 59,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 69,
                                    },
                                ),
                                ast_idx: 3,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 170,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            79,
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
                                                    81,
                                                ),
                                            },
                                            ty: 1,
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
                                                    86,
                                                ),
                                            },
                                            ty: 2,
                                        },
                                        ParameterDecl {
                                            pattern: ParameterPattern {
                                                pattern_expr_idx: 2,
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                            },
                                            colon: ColonToken {
                                                token_idx: TokenIdx(
                                                    90,
                                                ),
                                            },
                                            ty: 3,
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                84,
                                            ),
                                        },
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                88,
                                            ),
                                        },
                                    ],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            92,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            93,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    4,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            95,
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
                                            value: 60,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 70,
                                    },
                                ),
                                ast_idx: 4,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 171,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            117,
                                        ),
                                    },
                                    decls: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            118,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            119,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            121,
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
                            ast_idx: 171,
                            impl_block: ImplBlock(
                                Id {
                                    value: 25,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    162,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        164,
                                    ),
                                },
                            ),
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 172,
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
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 173,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            167,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    2,
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
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 174,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            178,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            180,
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
                                            value: 63,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 73,
                                    },
                                ),
                                ast_idx: 23,
                                expr_sheet: ExprSheet(
                                    Id {
                                        value: 175,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            297,
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
                                                    299,
                                                ),
                                            },
                                            ty: 1,
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
                                                    304,
                                                ),
                                            },
                                            ty: 2,
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                302,
                                            ),
                                        },
                                    ],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            306,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            307,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    3,
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            309,
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