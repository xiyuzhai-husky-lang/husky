[
    Ok(
        Signature::Type(
            TypeSignature::RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `start`,
                            ty: RawTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                        RegularStructFieldSignature {
                            ident: `end`,
                            ty: RawTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Impl(
            ImplSignature::TypeImpl(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    ty: RawTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeItem(
                TypeItemSignature::MethodFn(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterSignature {
                            liason: Pure,
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
                        nonself_regular_parameters: ExplicitParameterSignatures {
                            parameters: [],
                        },
                        return_ty: RawTerm(`mnist_classifier::geom2d::Vector2d`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeItem(
                TypeItemSignature::MethodFn(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterSignature {
                            liason: Pure,
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
                        nonself_regular_parameters: ExplicitParameterSignatures {
                            parameters: [
                                ExplicitParameterSignature {
                                    liason: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 76,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: RawTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
]