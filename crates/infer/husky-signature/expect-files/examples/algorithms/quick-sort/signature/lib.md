[
    Err(
        ExprError,
    ),
    Err(
        ExprError,
    ),
    Ok(
        Signature::Form(
            FormSignature::Function(
                FunctionSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                ty: Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                    parameters: RegularParameterSignatures {
                        parameters: [
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: ExplicitApplication(
                                    RawTermExplicitApplication(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                            },
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 17,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 17,
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
        ExprError,
    ),
    Err(
        ExprError,
    ),
]