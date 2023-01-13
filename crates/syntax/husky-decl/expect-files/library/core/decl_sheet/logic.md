Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`core::logic::LogicAnd`, `Structure`),
                            ast_idx: 0,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 4,
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
                                                            value: 10,
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
                                        ImplicitParameterDecl {
                                            ident: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 12,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    8,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    Some(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                7,
                                            ),
                                        },
                                    ],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Inductive(
                        InductiveTypeDecl {
                            path: TypePath(`core::logic::LogicOr`, `Inductive`),
                            ast_idx: 1,
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 5,
                                },
                            ),
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                    decls: [
                                        ImplicitParameterDecl {
                                            ident: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 10,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    26,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                        ImplicitParameterDecl {
                                            ident: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 12,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    30,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                    Some(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                29,
                                            ),
                                        },
                                    ],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                },
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)