Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::num::i8`, `Alien`),
                            ast_idx: 18,
                            expr_region: ExprRegion(
                                Id {
                                    value: 15,
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
                            ast_idx: 23,
                            expr_region: ExprRegion(
                                Id {
                                    value: 16,
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
                            ast_idx: 28,
                            expr_region: ExprRegion(
                                Id {
                                    value: 17,
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
                            ast_idx: 33,
                            expr_region: ExprRegion(
                                Id {
                                    value: 18,
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
                            ast_idx: 38,
                            expr_region: ExprRegion(
                                Id {
                                    value: 19,
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
                            ast_idx: 43,
                            expr_region: ExprRegion(
                                Id {
                                    value: 20,
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
                            ast_idx: 48,
                            expr_region: ExprRegion(
                                Id {
                                    value: 21,
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
                            ast_idx: 53,
                            expr_region: ExprRegion(
                                Id {
                                    value: 22,
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
                            ast_idx: 19,
                            impl_block: ImplBlock(
                                Id {
                                    value: 1,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        26,
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
                                        value: 24,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            31,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            32,
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
                                            34,
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
                            ast_idx: 24,
                            impl_block: ImplBlock(
                                Id {
                                    value: 3,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    82,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        84,
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
                                        value: 26,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            88,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            89,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            90,
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
                                            92,
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
                            ast_idx: 29,
                            impl_block: ImplBlock(
                                Id {
                                    value: 5,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    140,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        142,
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
                                        value: 28,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            146,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            147,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            148,
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
                                            150,
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
                            ast_idx: 34,
                            impl_block: ImplBlock(
                                Id {
                                    value: 7,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    197,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        199,
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
                                        value: 30,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            203,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            204,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            205,
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
                                            207,
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
                                    value: 9,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    255,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        257,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 31,
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
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            262,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            263,
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
                                            265,
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
                            ast_idx: 44,
                            impl_block: ImplBlock(
                                Id {
                                    value: 11,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    313,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        315,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
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
                                        value: 34,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            319,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            320,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            321,
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
                                            323,
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
                            ast_idx: 49,
                            impl_block: ImplBlock(
                                Id {
                                    value: 13,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    371,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        373,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 35,
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
                                        value: 36,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            377,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            378,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            379,
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
                                            381,
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
                            ast_idx: 54,
                            impl_block: ImplBlock(
                                Id {
                                    value: 15,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    428,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        430,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 37,
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
                                        value: 38,
                                    },
                                ),
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            434,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            435,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            436,
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
                                            438,
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