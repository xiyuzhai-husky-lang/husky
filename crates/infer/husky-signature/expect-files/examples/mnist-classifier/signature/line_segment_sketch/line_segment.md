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
                                        value: 212,
                                    },
                                ),
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
                        },
                        RegularStructFieldSignature {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 213,
                                    },
                                ),
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
                        },
                    ],
                },
            ),
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
                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                    ),
                },
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
                                                        value: 31,
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