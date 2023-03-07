[
    Err(
        OutputTypeRawTermError,
    ),
    Err(
        OutputTypeRawTermError,
    ),
    Ok(
        Signature::Form(
            FormSignature::Function(
                FunctionSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    parameters: RegularParameterSignatures {
                        parameters: [
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: ExplicitApplication(
                                    RawTermExplicitApplication(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: RawTerm(`core::option::Option core::num::f32`),
                },
            ),
        ),
    ),
]