[
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 50,
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