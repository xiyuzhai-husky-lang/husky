Ok(
    DeclSheet {
        decls: [
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`std::ops::Add`),
                        ast_idx: 3,
                        expr_region: ExprRegion(
                            Id {
                                value: 41,
                            },
                        ),
                        implicit_parameter_decl_list: Some(
                            ImplicitParameterDeclList {
                                langle: LeftAngleBracketOrLessThanToken {
                                    token_idx: TokenIdx(
                                        8,
                                    ),
                                },
                                implicit_parameters: [
                                    ImplicitParameterDecl {
                                        pattern: ImplicitParameterDeclPattern {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: Type0 {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 44,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        9,
                                                    ),
                                                },
                                            },
                                        },
                                        traits: None,
                                    },
                                ],
                                commas: [],
                                rangle: RightAngleBracketToken {
                                    token_idx: TokenIdx(
                                        10,
                                    ),
                                },
                            },
                        ),
                    },
                ),
            ),
        ],
    },
)