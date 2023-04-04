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
                TypeImplBlockSignature {
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
                TypeItemSignature::AssociatedFn(
                    TypeAssociatedFnSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        parameters: ExplicitParameterSignatures {
                            parameters: [
                                ExplicitParameterSignature {
                                    liason: Pure,
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
                                    liason: Pure,
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
                        return_ty: RawTerm(`mnist_classifier::geom2d::Point2d`),
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
                                            value: 76,
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
                                            value: 76,
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
                                            value: 76,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                                            value: 76,
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
    Ok(
        Signature::Impl(
            ImplSignature::TypeImpl(
                TypeImplBlockSignature {
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                                            value: 78,
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
                                                    value: 78,
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                                            value: 78,
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
                                                    value: 78,
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
                                            value: 78,
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
                                                    value: 78,
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
                                            value: 78,
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
                                                    value: 31,
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
                                            value: 78,
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
                                                    value: 78,
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
                                            value: 78,
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
                                                    value: 78,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                ExplicitParameterSignature {
                                    liason: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 31,
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
                TypeImplBlockSignature {
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
                                            value: 80,
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
                                                    value: 80,
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
                                            value: 80,
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
                        return_ty: RawTerm(`mnist_classifier::geom2d::RelativePoint2d`),
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                TypeImplBlockSignature {
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
                                            value: 81,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                                            value: 81,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                                            value: 81,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
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
                                            value: 81,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterSignatures {
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
]