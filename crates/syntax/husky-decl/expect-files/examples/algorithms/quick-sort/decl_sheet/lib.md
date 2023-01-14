Ok(
    DeclSheet {
        decls: [
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`quick_sort::quick_sort`, `Function`),
                            ast_idx: 31,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 40,
                                },
                            ),
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                    decls: [
                                        ImplicitParameterDecl {
                                            ident: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 41,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    4,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                },
                            ),
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        8,
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
                                                11,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        16,
                                    ),
                                },
                            },
                            curry_token: Err(
                                MissingCurry(
                                    TokenIdx(
                                        17,
                                    ),
                                ),
                            ),
                            output_ty: Err(
                                MissingOutputType,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        17,
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
                            path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                            ast_idx: 32,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 41,
                                },
                            ),
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            42,
                                        ),
                                    },
                                    decls: [
                                        ImplicitParameterDecl {
                                            ident: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 41,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    43,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            44,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            46,
                                        ),
                                    },
                                },
                            ),
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        47,
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
                                        ty: 3,
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
                                                57,
                                            ),
                                        },
                                        ty: 4,
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
                                                61,
                                            ),
                                        },
                                        ty: 5,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            59,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        63,
                                    ),
                                },
                            },
                            curry_token: Err(
                                MissingCurry(
                                    TokenIdx(
                                        64,
                                    ),
                                ),
                            ),
                            output_ty: Err(
                                MissingOutputType,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        64,
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
                            path: FormPath(`quick_sort::partition`, `Function`),
                            ast_idx: 33,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 42,
                                },
                            ),
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            103,
                                        ),
                                    },
                                    decls: [
                                        ImplicitParameterDecl {
                                            ident: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 41,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    104,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            105,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            107,
                                        ),
                                    },
                                },
                            ),
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        108,
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
                                                111,
                                            ),
                                        },
                                        ty: 3,
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
                                                118,
                                            ),
                                        },
                                        ty: 4,
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
                                                122,
                                            ),
                                        },
                                        ty: 5,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            116,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            120,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        124,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        125,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                6,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        127,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Feature(
                        FeatureDecl {
                            path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                            ast_idx: 35,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 43,
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Feature(
                        FeatureDecl {
                            path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                            ast_idx: 37,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 44,
                                },
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)