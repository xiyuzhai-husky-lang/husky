[
    Err(
        ExprError,
    ),
    Err(
        ExprError,
    ),
    Ok(
        Signature::Form(
            FormSignature::Fn(
                FnSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                    parameters: ExplicitParameterSignatures {
                        parameters: [
                            ExplicitParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: ExplicitApplication(
                                    RawTermExplicitApplication(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: RawTerm(`core::num::isize`),
                },
            ),
        ),
    ),
    Err(
        DeclExprError,
    ),
    Err(
        DeclExprError,
    ),
]