[
    Err(
        ReturnTypeRawTermError,
    ),
    Ok(
        Signature::Form(
            FormSignature::Fn(
                FnSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterSignatures {
                        parameters: [
                            ExplicitParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: ExplicitApplication(
                                    RawTermExplicitApplication(
                                        Id {
                                            value: 35,
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
    Err(
        ReturnTypeRawTermError,
    ),
]