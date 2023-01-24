[
    Ok(
        Type(
            RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        parameters: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 141,
                                    },
                                ),
                            ),
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        RegularStructFieldSignature {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        RegularStructFieldSignature {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 143,
                                    },
                                ),
                            ),
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        RegularStructFieldSignature {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 144,
                                    },
                                ),
                            ),
                            ty: Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Type(
            RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        parameters: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 146,
                                    },
                                ),
                            ),
                            ty: Application(
                                TermApplication(
                                    Id {
                                        value: 6,
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
                        parameters: [],
                    },
                    parameters: ParameterSignatures {
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
                        ],
                    },
                    output_ty: Application(
                        TermApplication {
                            function: Entity(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                            argument: Entity(
                                TypePath(`core::num::f32`, `Alien`),
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
    Err(
        FieldTypeTermError(
            0,
        ),
    ),
    Ok(
        Form(
            Function(
                FunctionSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        parameters: [],
                    },
                    parameters: ParameterSignatures {
                        parameters: [
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 15,
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
                                                    value: 15,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    output_ty: Entity(
                        TypePath(`core::raw_bits::r32`, `Alien`),
                    ),
                },
            ),
        ),
    ),
    Err(
        ParameterTypeTermError(
            0,
        ),
    ),
    Ok(
        ImplBlock(
            TypeImplBlock(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        parameters: [],
                    },
                    ty: Entity(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                Memo(
                    TypeMemoSignature {
                        output_ty: Application(
                            TermApplication {
                                function: Entity(
                                    TypePath(`core::vec::Vec`, `Alien`),
                                ),
                                argument: Entity(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            },
                        ),
                    },
                ),
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                Memo(
                    TypeMemoSignature {
                        output_ty: Entity(
                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                        ),
                    },
                ),
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                Memo(
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
                Memo(
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
                Memo(
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
                Memo(
                    TypeMemoSignature {
                        output_ty: Entity(
                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                        ),
                    },
                ),
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                Memo(
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
                Memo(
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
                Method(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            parameters: [],
                        },
                        parameters: ParameterSignatures {
                            parameters: [
                                ParameterSignature {
                                    pattern: ParameterSignaturePattern,
                                    ty: Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 9,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
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
                Method(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            parameters: [],
                        },
                        parameters: ParameterSignatures {
                            parameters: [
                                ParameterSignature {
                                    pattern: ParameterSignaturePattern,
                                    ty: Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 9,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        output_ty: Entity(
                            TypePath(`core::num::f32`, `Alien`),
                        ),
                    },
                ),
            ),
        ),
    ),
]