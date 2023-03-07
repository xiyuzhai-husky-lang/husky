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
                TypeImplSignature {
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
                TypeItemSignature::Method(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        parameters: RegularParameterSignatures {
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
                TypeItemSignature::Method(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        parameters: RegularParameterSignatures {
                            parameters: [
                                ParameterSignature {
                                    pattern: ParameterSignaturePattern,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 47,
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