Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i8`, `Alien`),
                            ast_idx: 17,
                            expr_region: ExprRegion(
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
                            ast_idx: 22,
                            expr_region: ExprRegion(
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
                            ast_idx: 26,
                            expr_region: ExprRegion(
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
                            ast_idx: 30,
                            expr_region: ExprRegion(
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
                            ast_idx: 34,
                            expr_region: ExprRegion(
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
                            ast_idx: 38,
                            expr_region: ExprRegion(
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
                            ast_idx: 42,
                            expr_region: ExprRegion(
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
                            ast_idx: 46,
                            expr_region: ExprRegion(
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
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
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
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                                ast_idx: 0,
                                expr_region: ExprRegion(
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
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 23,
                            impl_block: ImplBlock(
                                Id {
                                    value: 3,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    69,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        71,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 17,
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
                                        value: 3,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                                ast_idx: 2,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 18,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            75,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            76,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            77,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            79,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 27,
                            impl_block: ImplBlock(
                                Id {
                                    value: 5,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    114,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        116,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 19,
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
                                        value: 5,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                                ast_idx: 4,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 20,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            120,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            121,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            122,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            124,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 31,
                            impl_block: ImplBlock(
                                Id {
                                    value: 7,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    158,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        160,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
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
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                                ast_idx: 6,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 22,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            164,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            165,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            166,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            168,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 35,
                            impl_block: ImplBlock(
                                Id {
                                    value: 9,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    203,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        205,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 23,
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
                                        value: 9,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                                ast_idx: 8,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 24,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            209,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            210,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            211,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            213,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 39,
                            impl_block: ImplBlock(
                                Id {
                                    value: 11,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    248,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        250,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 25,
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
                                        value: 11,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                                ast_idx: 10,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 26,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            254,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            255,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            256,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            258,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 43,
                            impl_block: ImplBlock(
                                Id {
                                    value: 13,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    293,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        295,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
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
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 13,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                                ast_idx: 12,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 28,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            299,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            300,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            301,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            303,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 47,
                            impl_block: ImplBlock(
                                Id {
                                    value: 15,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    337,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        339,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 29,
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
                                        value: 15,
                                    },
                                ),
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 8,
                                        },
                                    ),
                                ),
                                ast_idx: 14,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 30,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            343,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            344,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            345,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            347,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
            ),
        ],
    },
)