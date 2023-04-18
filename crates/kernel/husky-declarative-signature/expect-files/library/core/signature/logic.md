[
    Ok(
        Signature::Type(
            TypeDeclarativeSignatureTemplate::Extern(
                ExternTypeDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeDeclarativeSignatureTemplate::Structure(
                StructureTypeDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
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
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeDeclarativeSignatureTemplate::Inductive(
                InductiveTypeDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
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
                },
            ),
        ),
    ),
]