[
    Ok(
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Move,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`core::basic::unit`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Move,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 53,
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
                                                value: 53,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`core::basic::unit`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Move,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 53,
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
                                                value: 53,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`core::num::isize`),
                },
            ),
        ),
    ),
]