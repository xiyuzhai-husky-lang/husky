Ok(
    DeclSheet {
        decls: [
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                            ast_idx: 22,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 158,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        20,
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
                                                22,
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
                                                26,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            24,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        28,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        29,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                2,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        31,
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