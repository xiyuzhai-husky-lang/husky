[
    Ok(
        Signature::Type(
            TypeDeclarativeSignature::RegularStruct(
                RegularStructTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `contour`,
                            ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                        },
                        RegularStructFieldSignature {
                            ident: `strokes`,
                            ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignature::Fn(
                FnDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatures {
                        parameters: [
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 78,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 54,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignature::Fn(
                FnDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatures {
                        parameters: [
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 78,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 54,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignature::Fn(
                FnDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatures {
                        parameters: [
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 54,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`core::num::i32`),
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignature::Fn(
                FnDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatures {
                        parameters: [
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 54,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`core::num::i32`),
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignature::Fn(
                FnDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatures {
                        parameters: [
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 54,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                },
            ),
        ),
    ),
    Ok(
        Signature::ImplBlock(
            ImplBlockDeclarativeSignature::TypeImpl(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::AssociatedFn(
                    TypeAssociatedFnDeclarativeSignature {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [
                                ExplicitParameterSignature {
                                    contract: Pure,
                                    ty: ExplicitApplication(
                                        DeclarativeTermExplicitApplication(
                                            Id {
                                                value: 29,
                                            },
                                        ),
                                    ),
                                },
                                ExplicitParameterSignature {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                ExplicitParameterSignature {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::MethodFn(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterSignature {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 82,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::ImplBlock(
            ImplBlockDeclarativeSignature::TypeImpl(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::AssociatedFn(
                    TypeAssociatedFnDeclarativeSignature {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [
                                ExplicitParameterSignature {
                                    contract: Pure,
                                    ty: ExplicitApplication(
                                        DeclarativeTermExplicitApplication(
                                            Id {
                                                value: 29,
                                            },
                                        ),
                                    ),
                                },
                                ExplicitParameterSignature {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 54,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                    },
                ),
            ),
        ),
    ),
]