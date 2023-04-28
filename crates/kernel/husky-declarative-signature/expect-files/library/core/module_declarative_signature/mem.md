[
    Ok(
        SignatureTemplate::Type(
            TypeDeclarativeSignatureTemplate::Extern(
                ExternDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                traits: [],
                            },
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::Type(
            TypeDeclarativeSignatureTemplate::Extern(
                ExternDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                traits: [],
                            },
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: Some(
                                    Invariant,
                                ),
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::Type(
            TypeDeclarativeSignatureTemplate::Extern(
                ExternDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                },
            ),
        ),
    ),
]