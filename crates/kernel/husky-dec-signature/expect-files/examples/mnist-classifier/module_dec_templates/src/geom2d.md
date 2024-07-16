```rust
[
    (
        ItemPath(`mnist_classifier::geom2d::Point2d`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `x`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `y`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativePoint2d`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `x`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `y`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `x`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `y`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::ClosedRange`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `min`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `max`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `xrange`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `yrange`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `xrange`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `yrange`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::from_i_shift28`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::AssocRitchie(
                        TypeAssocRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Point2d(0)::from_i_shift28`,
                                TypeItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::vector`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Point2d(0)::vector`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Point2d(0)::to`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::norm`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Point2d(0)::norm`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::dist`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Point2d(0)::dist`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::point`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Vector2d(0)::point`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::to`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Vector2d(0)::to`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Vector2d(0)::norm`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::dot`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Vector2d(0)::dot`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Vector2d(0)::cross`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Vector2d(0)::angle`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::basic::bool`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle_to`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::Vector2d(0)::angle_to`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::basic::bool`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::ClosedRange(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_range`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::ClosedRange(0)::relative_range`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_point`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::ClosedRange(0)::relative_point`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_point`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::BoundingBox(0)::relative_point`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::xmin`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::BoundingBox(0)::xmin`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::xmax`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::BoundingBox(0)::xmax`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::ymin`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::BoundingBox(0)::ymin`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::ymax`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::BoundingBox(0)::ymax`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```