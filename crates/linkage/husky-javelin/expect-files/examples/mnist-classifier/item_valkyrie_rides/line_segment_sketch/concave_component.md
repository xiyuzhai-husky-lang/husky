```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
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
                rides: [
                    ValkyrieRide::VecConstructor {
                        element_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                `core::vec::Vec(0)::cyclic_slice_leashed`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                `core::vec::Vec(0)::push`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::SelfLifetime,
                                    ),
                                    HirTermSymbolicVariableResolution::SelfLifetime,
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
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
        ItemPath::AssocItem(
            AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`,
                    TraitItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
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
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::CyclicSlice(0)::first`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                `core::slice::CyclicSlice(0)::start`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                `core::slice::CyclicSlice(0)::end`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                ],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::CyclicSlice(0)::start`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                `core::slice::CyclicSlice(0)::end`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                ],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::CyclicSlice(0)::first`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                `core::slice::CyclicSlice(0)::start`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                `core::slice::CyclicSlice(0)::end`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                ],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::CyclicSlice(0)::first`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                        path: JavPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                `<#derive _ as core::clone::Clone(0)>::clone`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::SelfType,
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                `core::slice::CyclicSlice(0)::last`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                ],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::CyclicSlice(0)::first`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                        path: JavPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                `<#derive _ as core::clone::Clone(0)>::clone`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::SelfType,
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                ],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::CyclicSlice(0)::last`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                        path: JavPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                `<#derive _ as core::clone::Clone(0)>::clone`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::SelfType,
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                ],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::CyclicSlice(0)::first`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                ],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::CyclicSlice(0)::last`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                ],
            },
        ),
    ),
]
```