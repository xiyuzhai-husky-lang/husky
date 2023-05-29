[
    Ok(
        SignatureTemplate::Type(
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `start`,
                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `end`,
                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                    ],
                },
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
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                        },
                        self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 90,
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
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                        },
                        self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 90,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            data: [
                                ExplicitParameterDeclarativeSignatureTemplate {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 80,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
]