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
                            ident: `x`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                        RegularStructFieldSignature {
                            ident: `y`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeDeclarativeSignature::RegularStruct(
                RegularStructTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `x`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                        RegularStructFieldSignature {
                            ident: `y`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeDeclarativeSignature::RegularStruct(
                RegularStructTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `x`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                        RegularStructFieldSignature {
                            ident: `y`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeDeclarativeSignature::RegularStruct(
                RegularStructTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `min`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                        RegularStructFieldSignature {
                            ident: `max`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeDeclarativeSignature::RegularStruct(
                RegularStructTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `xrange`,
                            ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                        },
                        RegularStructFieldSignature {
                            ident: `yrange`,
                            ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeDeclarativeSignature::RegularStruct(
                RegularStructTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `xrange`,
                            ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                        },
                        RegularStructFieldSignature {
                            ident: `yrange`,
                            ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                        },
                    ],
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
                    ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                                            value: 76,
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
                                            value: 76,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [
                                ExplicitParameterSignature {
                                    contract: Pure,
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
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                                            value: 76,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 76,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [
                                ExplicitParameterSignature {
                                    contract: Pure,
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
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                    ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                },
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
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
                            ],
                        },
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
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
                            ],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
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
                            ],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [
                                ExplicitParameterSignature {
                                    contract: Pure,
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
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
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
                            ],
                        },
                        return_ty: DeclarativeTerm(`core::num::i32`),
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
                                            value: 78,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
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
                                                    value: 31,
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
    Ok(
        Signature::ImplBlock(
            ImplBlockDeclarativeSignature::TypeImpl(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                },
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [
                                ExplicitParameterSignature {
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
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [
                                ExplicitParameterSignature {
                                    contract: Pure,
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
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativePoint2d`),
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 80,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                    ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                },
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
                                            value: 81,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 81,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 81,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
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
                                            value: 81,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
]