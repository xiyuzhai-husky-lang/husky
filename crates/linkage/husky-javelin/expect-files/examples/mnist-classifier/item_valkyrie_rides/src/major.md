```rust
[
    (
        ItemPath(`mnist_classifier::major::connected_components`),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_connected_component`),
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
                            symbol_map: [
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
                ],
            },
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`),
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
                            symbol_map: [
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
                ],
            },
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_raw_contours`),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_raw_contour`),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_line_segment_sketch`),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath(`mnist_classifier::major::major_concave_components`),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
]
```