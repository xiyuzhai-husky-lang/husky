Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i8`, `Alien`),
                            ast_idx: 17,
                            expr_page: ExprPage(
                                Id {
                                    value: 7,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i16`, `Alien`),
                            ast_idx: 21,
                            expr_page: ExprPage(
                                Id {
                                    value: 8,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i32`, `Alien`),
                            ast_idx: 25,
                            expr_page: ExprPage(
                                Id {
                                    value: 9,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i64`, `Alien`),
                            ast_idx: 29,
                            expr_page: ExprPage(
                                Id {
                                    value: 10,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::f8`, `Alien`),
                            ast_idx: 33,
                            expr_page: ExprPage(
                                Id {
                                    value: 11,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::f16`, `Alien`),
                            ast_idx: 37,
                            expr_page: ExprPage(
                                Id {
                                    value: 12,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::f32`, `Alien`),
                            ast_idx: 41,
                            expr_page: ExprPage(
                                Id {
                                    value: 13,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::f64`, `Alien`),
                            ast_idx: 45,
                            expr_page: ExprPage(
                                Id {
                                    value: 14,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 18,
                            impl_block: ImplBlock(
                                Id {
                                    value: 1,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 15,
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
                                            value: 1,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                ast_idx: 0,
                                expr_page: ExprPage(
                                    Id {
                                        value: 16,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            18,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            19,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            21,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                ast_idx: 1,
                                expr_page: ExprPage(
                                    Id {
                                        value: 17,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            37,
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
                                                    39,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            42,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            44,
                                        ),
                                    ),
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
                            ast_idx: 22,
                            impl_block: ImplBlock(
                                Id {
                                    value: 3,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    56,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        58,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 18,
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
                                            value: 2,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                ast_idx: 2,
                                expr_page: ExprPage(
                                    Id {
                                        value: 19,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            62,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            63,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            64,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            66,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 4,
                                    },
                                ),
                                ast_idx: 3,
                                expr_page: ExprPage(
                                    Id {
                                        value: 20,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            82,
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
                                                    84,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            86,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            87,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            89,
                                        ),
                                    ),
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
                            ast_idx: 26,
                            impl_block: ImplBlock(
                                Id {
                                    value: 5,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    101,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        103,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 21,
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
                                            value: 3,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 5,
                                    },
                                ),
                                ast_idx: 4,
                                expr_page: ExprPage(
                                    Id {
                                        value: 22,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            107,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            108,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            109,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            111,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                ast_idx: 5,
                                expr_page: ExprPage(
                                    Id {
                                        value: 23,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            126,
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
                                                    128,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            130,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            131,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            133,
                                        ),
                                    ),
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
                            ast_idx: 30,
                            impl_block: ImplBlock(
                                Id {
                                    value: 7,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    145,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        147,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 24,
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
                                            value: 4,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                ast_idx: 6,
                                expr_page: ExprPage(
                                    Id {
                                        value: 25,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            151,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            152,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            153,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            155,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 8,
                                    },
                                ),
                                ast_idx: 7,
                                expr_page: ExprPage(
                                    Id {
                                        value: 26,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            171,
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
                                                    173,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            175,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            176,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            178,
                                        ),
                                    ),
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
                            ast_idx: 34,
                            impl_block: ImplBlock(
                                Id {
                                    value: 9,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    190,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        192,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 27,
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
                                            value: 5,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 9,
                                    },
                                ),
                                ast_idx: 8,
                                expr_page: ExprPage(
                                    Id {
                                        value: 28,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            196,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            197,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            198,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            200,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 10,
                                    },
                                ),
                                ast_idx: 9,
                                expr_page: ExprPage(
                                    Id {
                                        value: 29,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            216,
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
                                                    218,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            220,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            221,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            223,
                                        ),
                                    ),
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
                            ast_idx: 38,
                            impl_block: ImplBlock(
                                Id {
                                    value: 11,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    235,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        237,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 30,
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
                                            value: 6,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 11,
                                    },
                                ),
                                ast_idx: 10,
                                expr_page: ExprPage(
                                    Id {
                                        value: 31,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            241,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            242,
                                        ),
                                    },
                                },
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
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            245,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 12,
                                    },
                                ),
                                ast_idx: 11,
                                expr_page: ExprPage(
                                    Id {
                                        value: 32,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            261,
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
                                                    263,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            265,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            266,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            268,
                                        ),
                                    ),
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
                            ast_idx: 42,
                            impl_block: ImplBlock(
                                Id {
                                    value: 13,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    280,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        282,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 33,
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
                                            value: 7,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 13,
                                    },
                                ),
                                ast_idx: 12,
                                expr_page: ExprPage(
                                    Id {
                                        value: 34,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            286,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            287,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            288,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            290,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 14,
                                    },
                                ),
                                ast_idx: 13,
                                expr_page: ExprPage(
                                    Id {
                                        value: 35,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            305,
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
                                                    307,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            309,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            310,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            312,
                                        ),
                                    ),
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
                            ast_idx: 46,
                            impl_block: ImplBlock(
                                Id {
                                    value: 15,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    324,
                                ),
                            },
                            ty: 0,
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        326,
                                    ),
                                },
                            ),
                            expr_page: ExprPage(
                                Id {
                                    value: 36,
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
                                            value: 8,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 15,
                                    },
                                ),
                                ast_idx: 14,
                                expr_page: ExprPage(
                                    Id {
                                        value: 37,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            330,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            331,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            332,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    0,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            334,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Err(
                ImplBlockErr,
            ),
            Ok(
                AssociatedItem(
                    TypeAsTraitItem(
                        Method(
                            TypeAsTraitMethodDecl {
                                path: None,
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 16,
                                    },
                                ),
                                ast_idx: 15,
                                expr_page: ExprPage(
                                    Id {
                                        value: 38,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            350,
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
                                                    352,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            354,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            355,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    1,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            357,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)