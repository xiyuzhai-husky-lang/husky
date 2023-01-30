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
                                        value: 256,
                                    },
                                ),
                            ),
                            ty: Application(
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                        RegularStructFieldSignature {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 301,
                                    },
                                ),
                            ),
                            ty: Application(
                                TermApplication(
                                    Id {
                                        value: 18,
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
                                ty: Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 33,
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
                                                    value: 13,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    output_ty: Entity(
                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                    ),
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
                                ty: Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 33,
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
                                                    value: 13,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    output_ty: Entity(
                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                    ),
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
                                                    value: 9,
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
                                                    value: 13,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    output_ty: Entity(
                        TypePath(`core::num::i32`, `Alien`),
                    ),
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
                                                    value: 9,
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
                                                    value: 9,
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
                                                    value: 13,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    output_ty: Entity(
                        TypePath(`core::num::i32`, `Alien`),
                    ),
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
                                                    value: 13,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    output_ty: Application(
                        TermApplication {
                            function: Entity(
                                TypePath(`core::vec::Vec`, `Alien`),
                            ),
                            argument: Entity(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        },
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
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                TypeItemSignature::Method(
                    TypeMethodSignature {
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
                                                        value: 9,
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
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    },
                ),
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                TypeItemSignature::Method(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        parameters: ParameterSignatures {
                            parameters: [],
                        },
                        output_ty: Entity(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    },
                ),
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
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                        output_ty: Application(
                            TermApplication {
                                function: Entity(
                                    TypePath(`core::vec::Vec`, `Alien`),
                                ),
                                argument: Entity(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                TypeItemSignature::Memo(
                    TypeMemoSignature {
                        output_ty: Entity(
                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        ),
                    },
                ),
            ),
        ),
    ),
    Ok(
        AssociatedItem(
            TypeItem(
                TypeItemSignature::Method(
                    TypeMethodSignature {
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
                                                        value: 13,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        output_ty: Entity(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                    },
                ),
            ),
        ),
    ),
]