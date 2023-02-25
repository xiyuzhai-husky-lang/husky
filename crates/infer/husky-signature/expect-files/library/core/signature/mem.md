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
                                term_symbol: TermSymbol(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                ty: Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Covariant,
                                ),
                                term_symbol: TermSymbol(
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
                                term_symbol: TermSymbol(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                ty: Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
                                annotated_variance: Some(
                                    Invariant,
                                ),
                                term_symbol: TermSymbol(
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