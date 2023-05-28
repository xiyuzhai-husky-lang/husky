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
                                        value: 8,
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
                                        value: 8,
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
        SignatureTemplate::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                TraitForTypeImplBlockDeclarativeSignatureTemplate {
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
                    trai: DeclarativeTerm(`core::marker::Copy`),
                    ty: DeclarativeTerm(`core::mem::Leash t`),
                },
            ),
        ),
    ),
]