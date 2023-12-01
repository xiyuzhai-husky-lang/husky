[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `x`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `y`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativePoint2d`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `x`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `y`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `x`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `y`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `min`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `max`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `xrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `yrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `xrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `yrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                        TypeAssociatedFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::vector`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::vector`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::norm`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::norm`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::point`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::point`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::to`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::to`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::basic::bool`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle_to`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle_to`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::basic::bool`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativePoint2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmin`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmin`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmax`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmax`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
]