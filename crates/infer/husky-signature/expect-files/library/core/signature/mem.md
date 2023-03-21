[
    Ok(
        Signature::Type(
            TypeSignature::Foreign(
                ExternTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 66,
                                            },
                                        ),
                                    ),
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                ty: Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
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
            TypeSignature::Foreign(
                ExternTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 66,
                                            },
                                        ),
                                    ),
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Invariant,
                                ),
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                ty: Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
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
            TypeSignature::Foreign(
                ExternTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                ty: Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
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