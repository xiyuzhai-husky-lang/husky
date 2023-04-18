[
    Ok(
        Signature::Type(
            TypeDeclarativeSignature::Extern(
                ExternTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterSignature {
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
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 1,
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
        Signature::Type(
            TypeDeclarativeSignature::Extern(
                ExternTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterSignature {
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
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Invariant,
                                ),
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 1,
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
        Signature::Type(
            TypeDeclarativeSignature::Extern(
                ExternTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 1,
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