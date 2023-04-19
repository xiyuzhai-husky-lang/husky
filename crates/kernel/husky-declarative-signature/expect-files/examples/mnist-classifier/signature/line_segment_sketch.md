[
    Ok(
        Signature::Type(
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructTypeDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
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
            FormDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            FormDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            FormDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            FormDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            FormDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                    TypeAssociatedFnDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                            data: [],
                        },
                        parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::Memo(
                    TypeMemoizedFieldDeclarativeSignature {
                        return_ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::Memo(
                    TypeMemoizedFieldDeclarativeSignature {
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                    TypeAssociatedFnDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                            data: [],
                        },
                        parameters: ExplicitParameterDeclarativeSignatureTemplates {
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