Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::logic::Prop`, `Alien`),
                            ast_idx: 0,
                            expr_page: ExprPage(
                                Id {
                                    value: 4,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`core::logic::LogicAnd`, `Structure`),
                            ast_idx: 1,
                            expr_page: ExprPage(
                                Id {
                                    value: 5,
                                },
                            ),
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                    implicit_parameters: [
                                        ImplicitParameterDecl {
                                            pattern: ImplicitParameterDeclPattern {
                                                symbol: 0,
                                                variant: Type0 {
                                                    ident_token: IdentifierToken {
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
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                        ImplicitParameterDecl {
                                            pattern: ImplicitParameterDeclPattern {
                                                symbol: 1,
                                                variant: Type0 {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            13,
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
                                                11,
                                            ),
                                        },
                                    ],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            15,
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
                            ast_idx: 2,
                            expr_page: ExprPage(
                                Id {
                                    value: 6,
                                },
                            ),
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            29,
                                        ),
                                    },
                                    implicit_parameters: [
                                        ImplicitParameterDecl {
                                            pattern: ImplicitParameterDeclPattern {
                                                symbol: 0,
                                                variant: Type0 {
                                                    ident_token: IdentifierToken {
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
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                        ImplicitParameterDecl {
                                            pattern: ImplicitParameterDeclPattern {
                                                symbol: 1,
                                                variant: Type0 {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            35,
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
                                                33,
                                            ),
                                        },
                                    ],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            37,
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