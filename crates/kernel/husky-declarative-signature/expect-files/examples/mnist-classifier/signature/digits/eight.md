[
    Ok(
        Signature::Form(
            FormDeclarativeSignatureTemplate::Val(
                ValDeclarativeSignatureTemplate {
                    return_ty: DeclarativeTerm(`mnist_classifier::fermi::FermiMatchResult`),
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignatureTemplate::Val(
                ValDeclarativeSignatureTemplate {
                    return_ty: DeclarativeTerm(`core::option::Option mnist::MnistLabel`),
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
]