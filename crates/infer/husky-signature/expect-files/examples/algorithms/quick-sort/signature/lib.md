[
    Err(
        ExprError,
    ),
    Err(
        ExprError,
    ),
    Ok(
        Signature::Form(
            FormSignature::Function(
                FunctionSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: None,
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
                    parameters: RegularParameterSignatures {
                        parameters: [
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: Application(
                                    TermApplication(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                            },
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: Term(`core::num::isize`),
                },
            ),
        ),
    ),
    Err(
        ExprError,
    ),
    Err(
        ExprError,
    ),
]