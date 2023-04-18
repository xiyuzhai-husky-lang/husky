[
    Ok(
        Signature::Form(
            FormDeclarativeSignature::Fn(
                FnDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatures {
                        parameters: [
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 83,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`core::basic::bool`),
                },
            ),
        ),
    ),
]