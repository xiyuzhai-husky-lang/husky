[
    Err(
        OutputTypeTermError,
    ),
    Err(
        OutputTypeTermError,
    ),
    Ok(
        Form(
            Function(
                FunctionSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        parameters: [],
                    },
                    parameters: ParameterSignatures {
                        parameters: [
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: Application(
                                    TermApplication(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                    output_ty: Application(
                        TermApplication {
                            m: Entity(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                            n: Entity(
                                TypePath(`core::num::f32`, `Alien`),
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
]