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
                            ast_idx: 21,
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
                            ast_idx: 25,
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
                            ast_idx: 29,
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
                            ast_idx: 33,
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
                            ast_idx: 37,
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
                            ast_idx: 41,
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
                            ast_idx: 45,
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
                            ty: 0,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
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
            Err(
                UnableToParseImplBlockDeclForTyAsTraitMethodDecl,
            ),
        ],
    },
)