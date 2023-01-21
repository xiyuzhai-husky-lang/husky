[
    Trait(
        TraitSignature {
            implicit_parameters: ImplicitParameterSignatures {
                parameters: [
                    ImplicitParameterSignature {
                        term_symbol: TermSymbol(
                            Id {
                                value: 1,
                            },
                        ),
                        ty: Success(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                        ),
                        traits: [],
                    },
                ],
            },
            term_sheet: SignatureTermSheet {
                term_symbol_region: TermSymbolRegion {
                    registry: TermSymbolRegistry {
                        tys: [
                            Ok(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ],
                    },
                    inherited_symbol_terms: [],
                    current_symbol_terms: [
                        TermSymbol(
                            Id {
                                value: 1,
                            },
                        ),
                    ],
                },
            },
        },
    ),
]