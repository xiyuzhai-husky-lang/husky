Ok(
    DeclSheet {
        decls: [
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                            ast_idx: 22,
                            expr_region: ExprRegion(
                                Id {
                                    value: 150,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        21,
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
                                                23,
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
                                                27,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        29,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        30,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                2,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        32,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)