[
    Ok(
        Signature::Form(
            FormSignature::Function(
                FormFnSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    parameters: RegularParameterSignatures {
                        parameters: [
                            RegularParameterSignature {
                                pattern: ParameterSignaturePattern,
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
                            RegularParameterSignature {
                                pattern: ParameterSignaturePattern,
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
                    return_ty: RawTerm(`core::basic::bool`),
                },
            ),
        ),
    ),
]