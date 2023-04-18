[
    Ok(
        Signature::Form(
            FormDeclarativeSignature::Val(
                ValDeclarativeSignature {
                    return_ty: DeclarativeTerm(`mnist_classifier::fermi::FermiMatchResult`),
                },
            ),
        ),
    ),
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
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 36,
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`core::option::Option core::num::f32`),
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignature::Val(
                ValDeclarativeSignature {
                    return_ty: DeclarativeTerm(`core::option::Option mnist::MnistLabel`),
                },
            ),
        ),
    ),
]