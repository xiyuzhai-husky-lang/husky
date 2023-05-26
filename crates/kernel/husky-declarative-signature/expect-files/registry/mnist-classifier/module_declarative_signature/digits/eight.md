[
    Ok(
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Val(
                ValDeclarativeSignatureTemplate {
                    return_ty: DeclarativeTerm(`mnist_classifier::fermi::FermiMatchResult`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Val(
                ValDeclarativeSignatureTemplate {
                    return_ty: DeclarativeTerm(`core::option::Option mnist::MnistLabel`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ExplicitParameterDeclarativeSignatureTemplate {
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