[
    Ok(
        Signature::Type(
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `x`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                        RegularStructFieldDeclarativeSignatureTemplate {
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
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `x`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                        RegularStructFieldDeclarativeSignatureTemplate {
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
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `x`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                        RegularStructFieldDeclarativeSignatureTemplate {
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
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `min`,
                            ty: DeclarativeTerm(`core::num::f32`),
                        },
                        RegularStructFieldDeclarativeSignatureTemplate {
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
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `xrange`,
                            ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                        },
                        RegularStructFieldDeclarativeSignatureTemplate {
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
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldDeclarativeSignatureTemplate {
                            ident: `xrange`,
                            ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                        },
                        RegularStructFieldDeclarativeSignatureTemplate {
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
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockDeclarativeSignatureTemplate {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                    TypeAssociatedFnDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockDeclarativeSignatureTemplate {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterSignature {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 79,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            parameters: [
                                ExplicitParameterSignature {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
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
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterSignature {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 79,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            parameters: [
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
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockDeclarativeSignatureTemplate {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockDeclarativeSignatureTemplate {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            parameters: [],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
]