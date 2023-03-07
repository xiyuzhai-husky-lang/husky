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
                            ident: `x`,
                            ty: RawTerm(`core::num::f32`),
                        },
                        RegularStructFieldSignature {
                            ident: `y`,
                            ty: RawTerm(`core::num::f32`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeSignature::RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `x`,
                            ty: RawTerm(`core::num::f32`),
                        },
                        RegularStructFieldSignature {
                            ident: `y`,
                            ty: RawTerm(`core::num::f32`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeSignature::RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `x`,
                            ty: RawTerm(`core::num::f32`),
                        },
                        RegularStructFieldSignature {
                            ident: `y`,
                            ty: RawTerm(`core::num::f32`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeSignature::RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `min`,
                            ty: RawTerm(`core::num::f32`),
                        },
                        RegularStructFieldSignature {
                            ident: `max`,
                            ty: RawTerm(`core::num::f32`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeSignature::RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    fields: [],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeSignature::RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `xrange`,
                            ty: RawTerm(`mnist_classifier::geom2d::ClosedRange`),
                        },
                        RegularStructFieldSignature {
                            ident: `yrange`,
                            ty: RawTerm(`mnist_classifier::geom2d::ClosedRange`),
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
                    ty: RawTerm(`mnist_classifier::geom2d::Point2d`),
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
                            parameters: [
                                ParameterSignature {
                                    pattern: ParameterSignaturePattern,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                ParameterSignature {
                                    pattern: ParameterSignaturePattern,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: RawTerm(`mnist_classifier::geom2d::Point2d`),
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
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
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
    Ok(
        Signature::Impl(
            ImplSignature::TypeImpl(
                TypeImplSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    ty: RawTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        return_ty: RawTerm(`mnist_classifier::geom2d::Point2d`),
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
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
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
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
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
                                                    value: 49,
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
                                                    value: 49,
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
                                                    value: 1,
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
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: RawTerm(`core::num::i32`),
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
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                ParameterSignature {
                                    pattern: ParameterSignaturePattern,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 1,
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
    Ok(
        Signature::Impl(
            ImplSignature::TypeImpl(
                TypeImplSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    ty: RawTerm(`mnist_classifier::geom2d::BoundingBox`),
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
                            parameters: [
                                ParameterSignature {
                                    pattern: ParameterSignaturePattern,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: RawTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
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
                        return_ty: RawTerm(`mnist_classifier::geom2d::RelativePoint2d`),
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
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
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
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
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
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
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
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
                    },
                ),
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
                    ty: RawTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
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
                        return_ty: RawTerm(`core::num::f32`),
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
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
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
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
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
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
]