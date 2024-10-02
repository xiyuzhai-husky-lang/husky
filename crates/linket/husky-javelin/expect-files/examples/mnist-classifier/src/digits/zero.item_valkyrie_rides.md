```rust
[
    (
        ItemPath(`mnist_classifier::digits::zero::open_one_match`),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide::VecConstructor {
                        element_ty: HirType::Ritchie(
                            HirRitchieType {
                                ritchie_ty_kind: RitchieTypeKind::Item(
                                    RitchieItemKind::Fn,
                                ),
                                parameters: HirRitchieParameters {
                                    data: [
                                        HirRitchieParameter::Simple(
                                            HirRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                        ),
                                    ],
                                },
                                return_ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::option::Option`, `Enum`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
    ),
    (
        ItemPath(`mnist_classifier::digits::zero::almost_closed`),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath(`mnist_classifier::digits::zero::is_zero`),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::vec::Vec(0)::ilen`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            path: ItemPath(`core::vec::Vec(0)::ilen`),
                            context: HirTypeContext {
                                comptime_var_overrides: [],
                            },
                            variable_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::vec::Vec(0)::ilen`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            path: ItemPath(`core::vec::Vec(0)::ilen`),
                            context: HirTypeContext {
                                comptime_var_overrides: [],
                            },
                            variable_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide::VecConstructor {
                        element_ty: HirType::Ritchie(
                            HirRitchieType {
                                ritchie_ty_kind: RitchieTypeKind::Item(
                                    RitchieItemKind::Fn,
                                ),
                                parameters: HirRitchieParameters {
                                    data: [
                                        HirRitchieParameter::Simple(
                                            HirRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                        ),
                                    ],
                                },
                                return_ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::option::Option`, `Enum`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                            },
                        ),
                    },
                    ValkyrieRide::PathLeading {
                        path: JavPath::Form(
                            MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                Gn,
                            )`),
                        ),
                        hir_instantiation: HirInstantiation {
                            path: ItemPath(`malamute::narrow_down`),
                            context: HirTypeContext {
                                comptime_var_overrides: [
                                    (
                                        MajorItem(
                                            Form(
                                                MajorFormPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 116,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Some(
                                            Type(
                                                PathLeading(
                                                    HirTypePathLeading(
                                                        Id {
                                                            value: 47,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ],
                            },
                            variable_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVariable::Compterm(
                                        HirComptermTemplateVariable {
                                            ty: HirType::Variable(
                                                HirTypeTemplateVariable::Type {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: HirComptermTemplateVariableIndex::Other {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Poly,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            HirCompterm::TypeVariant(
                                                TypeVariantPath(`mnist::MnistLabel::Zero`),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: None,
                        },
                    },
                    ValkyrieRide::TypeDefault {
                        ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                    },
                ],
            },
        ),
    ),
]
```