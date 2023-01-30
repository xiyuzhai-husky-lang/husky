[
    Ok(
        Type(
            TypeSignature::RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 147,
                                    },
                                ),
                            ),
                            ty: Application(
                                TermApplication(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        },
                        RegularStructFieldSignature {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 363,
                                    },
                                ),
                            ),
                            ty: Application(
                                TermApplication(
                                    Id {
                                        value: 13,
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Form(
            Function(
                FunctionSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    parameters: ParameterSignatures {
                        parameters: [
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: Application(
                                    TermApplication(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                ),
                            },
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: Application(
                                    TermApplication(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                    output_ty: Entity(
                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    Ok(
        ImplBlock(
            TypeImplBlock(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    ty: Entity(
                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                TypeItemSignature::Memo(
                    TypeMemoSignature {
                        output_ty: Entity(
                            TypePath(`core::num::f32`, `Alien`),
                        ),
                    },
                ),
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                TypeItemSignature::Memo(
                    TypeMemoSignature {
                        output_ty: Entity(
                            TypePath(`core::num::f32`, `Alien`),
                        ),
                    },
                ),
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                TypeItemSignature::Memo(
                    TypeMemoSignature {
                        output_ty: Entity(
                            TypePath(`core::num::f32`, `Alien`),
                        ),
                    },
                ),
            ),
        ),
    ),
]