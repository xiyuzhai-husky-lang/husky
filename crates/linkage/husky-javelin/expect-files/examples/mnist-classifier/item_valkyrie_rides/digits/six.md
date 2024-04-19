```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
            ),
        ),
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
            ),
        ),
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::Form(
                            FormPath(`malamute::narrow_down`, `Ritchie(
                                Gn,
                            )`),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Comptime,
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
                                    HirTemplateVariable::Const(
                                        HirConstTemplateVariable {
                                            ty: HirType::Variable(
                                                HirTypeTemplateVariable::Type {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: HirConstTemplateVariableIndex::Other {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 269,
                                                        },
                                                    ),
                                                ),
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
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::digits::six::upmost`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::digits::six::bottom1`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
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
]
```