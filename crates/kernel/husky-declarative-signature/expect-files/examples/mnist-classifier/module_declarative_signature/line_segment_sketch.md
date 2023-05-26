[
    Err(
        FieldTypeDeclarativeTermError(
            0,
        ),
    ),
    Ok(
        SignatureTemplate::Type(
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `contour`,
                            ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                        },
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `strokes`,
                            ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ExplicitParameterDeclarativeSignatureTemplate {
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
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 56,
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
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ExplicitParameterDeclarativeSignatureTemplate {
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
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 56,
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
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 46,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 56,
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
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 46,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 46,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 56,
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
        SignatureTemplate::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        data: [
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 56,
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
        SignatureTemplate::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                TraitForTypeImplBlockDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    trai: DeclarativeTerm(`core::visual::Visualize`),
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                    TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 86,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            data: [],
                        },
                        return_ty: DeclarativeTerm(`core::basic::unit`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        SignatureTemplate::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                    TypeAssociatedFnDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            data: [
                                ExplicitParameterDeclarativeSignatureTemplate {
                                    contract: Pure,
                                    ty: ExplicitApplication(
                                        DeclarativeTermExplicitApplication(
                                            Id {
                                                value: 29,
                                            },
                                        ),
                                    ),
                                },
                                ExplicitParameterDeclarativeSignatureTemplate {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                ExplicitParameterDeclarativeSignatureTemplate {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 46,
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
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 86,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            data: [],
                        },
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        SignatureTemplate::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                TraitForTypeImplBlockDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    trai: DeclarativeTerm(`core::visual::Visualize`),
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                    TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 87,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            data: [],
                        },
                        return_ty: DeclarativeTerm(`core::visual::Html`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        SignatureTemplate::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MemoizedField(
                    TypeMemoizedFieldDeclarativeSignatureTemplate {
                        return_ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MemoizedField(
                    TypeMemoizedFieldDeclarativeSignatureTemplate {
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                    TypeAssociatedFnDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            data: [
                                ExplicitParameterDeclarativeSignatureTemplate {
                                    contract: Pure,
                                    ty: ExplicitApplication(
                                        DeclarativeTermExplicitApplication(
                                            Id {
                                                value: 29,
                                            },
                                        ),
                                    ),
                                },
                                ExplicitParameterDeclarativeSignatureTemplate {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 56,
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