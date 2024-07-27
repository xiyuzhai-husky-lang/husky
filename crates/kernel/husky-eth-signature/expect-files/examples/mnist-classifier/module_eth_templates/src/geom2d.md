```rust
[
    (
        ItemPath(`mnist_classifier::geom2d::Point2d`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`Point2d`),
                                    ident: `x`,
                                    ty: EthTerm(`f32`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`Point2d`),
                                    ident: `y`,
                                    ty: EthTerm(`f32`),
                                },
                            ],
                            instance_constructor_ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Point2d`),
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
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`RelativePoint2d`),
                                    ident: `x`,
                                    ty: EthTerm(`f32`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`RelativePoint2d`),
                                    ident: `y`,
                                    ty: EthTerm(`f32`),
                                },
                            ],
                            instance_constructor_ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`RelativePoint2d`),
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
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`Vector2d`),
                                    ident: `x`,
                                    ty: EthTerm(`f32`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`Vector2d`),
                                    ident: `y`,
                                    ty: EthTerm(`f32`),
                                },
                            ],
                            instance_constructor_ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Vector2d`),
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
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`ClosedRange`),
                                    ident: `min`,
                                    ty: EthTerm(`f32`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`ClosedRange`),
                                    ident: `max`,
                                    ty: EthTerm(`f32`),
                                },
                            ],
                            instance_constructor_ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`ClosedRange`),
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
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`BoundingBox`),
                                    ident: `xrange`,
                                    ty: EthTerm(`ClosedRange`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`BoundingBox`),
                                    ident: `yrange`,
                                    ty: EthTerm(`ClosedRange`),
                                },
                            ],
                            instance_constructor_ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`ClosedRange`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`ClosedRange`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`BoundingBox`),
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
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`RelativeBoundingBox`),
                                    ident: `xrange`,
                                    ty: EthTerm(`ClosedRange`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`RelativeBoundingBox`),
                                    ident: `yrange`,
                                    ty: EthTerm(`ClosedRange`),
                                },
                            ],
                            instance_constructor_ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`ClosedRange`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`ClosedRange`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`RelativeBoundingBox`),
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
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::geom2d::Point2d(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`Point2d`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::from_i_shift28`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    AssocRitchie(
                        TypeAssocRitchieEthTemplate(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::vector`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::norm`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Point2d(0)::dist`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 6,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::geom2d::Vector2d(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`Vector2d`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::point`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 7,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::to`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 9,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::dot`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 12,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle_to`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::ClosedRange(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::geom2d::ClosedRange(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`ClosedRange`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_range`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 15,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_point`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 16,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::geom2d::BoundingBox(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`BoundingBox`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 17,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_point`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::xmin`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::xmax`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::ymin`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::ymax`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`RelativeBoundingBox`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
]
```