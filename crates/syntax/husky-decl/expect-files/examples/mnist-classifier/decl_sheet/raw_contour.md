Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ast_idx: 199,
                            expr_region: ExprRegion(
                                Id {
                                    value: 183,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    29,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 179,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            31,
                                        ),
                                    },
                                    ty: 1,
                                },
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 147,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            35,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            36,
                                        ),
                                    },
                                    ty: 4,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        34,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        40,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    41,
                                ),
                            },
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Enum(
                        EnumTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                            ast_idx: 201,
                            expr_region: ExprRegion(
                                Id {
                                    value: 184,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                            ast_idx: 206,
                            expr_region: ExprRegion(
                                Id {
                                    value: 185,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        405,
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
                                                407,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                411,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            409,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        413,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        414,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 2,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        416,
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
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                            ast_idx: 207,
                            expr_region: ExprRegion(
                                Id {
                                    value: 186,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        430,
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
                                                432,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                436,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            434,
                                        ),
                                    },
                                ],
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
                                OutputTypeExpr {
                                    expr: 2,
                                },
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
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                            ast_idx: 208,
                            expr_region: ExprRegion(
                                Id {
                                    value: 187,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        451,
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
                                                453,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                457,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            455,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        459,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        460,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 2,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        462,
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
                            path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                            ast_idx: 209,
                            expr_region: ExprRegion(
                                Id {
                                    value: 188,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        476,
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
                                                478,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                482,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 2,
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                486,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            480,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            484,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            488,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        489,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        490,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        492,
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
                            path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                            ast_idx: 210,
                            expr_region: ExprRegion(
                                Id {
                                    value: 189,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        622,
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
                                                624,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                628,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            626,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        630,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        631,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 2,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        633,
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
                            path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                            ast_idx: 211,
                            expr_region: ExprRegion(
                                Id {
                                    value: 190,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        686,
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
                                                688,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                692,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 2,
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                696,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 3,
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                700,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            690,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            694,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            698,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            702,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        703,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        704,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 4,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        706,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                            ast_idx: 212,
                            expr_region: ExprRegion(
                                Id {
                                    value: 191,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    913,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 230,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            914,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            915,
                                        ),
                                    },
                                    ty: 0,
                                },
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 231,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            918,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            919,
                                        ),
                                    },
                                    ty: 1,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        917,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        921,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    922,
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
                            path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                            ast_idx: 213,
                            expr_region: ExprRegion(
                                Id {
                                    value: 192,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        925,
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
                                                927,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        931,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        932,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        934,
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
                            path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                            ast_idx: 214,
                            expr_region: ExprRegion(
                                Id {
                                    value: 193,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        990,
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
                                                992,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        995,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        996,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 4,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        1000,
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
                            ast_idx: 200,
                            impl_block: ImplBlock(
                                Id {
                                    value: 18,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    42,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        44,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 194,
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
                                            value: 64,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 74,
                                    },
                                ),
                                ast_idx: 27,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 195,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            49,
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
                                            value: 65,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 75,
                                    },
                                ),
                                ast_idx: 28,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 196,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            60,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            62,
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
                                            value: 66,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 76,
                                    },
                                ),
                                ast_idx: 29,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 197,
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
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            182,
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
                                            value: 67,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 77,
                                    },
                                ),
                                ast_idx: 30,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 198,
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            202,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            204,
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
                                        value: 78,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 68,
                                        },
                                    ),
                                ),
                                ast_idx: 31,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 199,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            331,
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
                                                    333,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken {
                                                token_idx: TokenIdx(
                                                    337,
                                                ),
                                            },
                                            ty: 1,
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                335,
                                            ),
                                        },
                                    ],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            339,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            340,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 2,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            342,
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