```rust
ValkyrieJavelinPantheon {
    package_path: PackagePath {
        toolchain: Toolchain {
            data: ToolchainData::Local {
                library_path: "../../../library",
            },
        },
        name: `mnist-classifier`,
        data: PackagePathSource::Local {
            path: "../../../examples/mnist-classifier",
        },
    },
    instantiation_map: {
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::start`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::start`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::first`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::first`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                template_arguments: [
                                                                    JavTemplateArgument::Type(
                                                                        JavType::PathLeading(
                                                                            JavTypePathLeading {
                                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                                template_arguments: [],
                                                                            },
                                                                        ),
                                                                    ),
                                                                ],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::collect_leashes`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::collect_leashes`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::first`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::first`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::Type(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`malamute::Class`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: None,
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::TypeDefault {
                    ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::Ritchie(
                        JavRitchieType {
                            parameters: [
                                JavRitchieParameter {
                                    contract: Pure,
                                    parameter_ty: PathLeading(
                                        JavTypePathLeading(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                },
                            ],
                            return_ty: JavType::PathLeading(
                                JavTypePathLeading {
                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                    template_arguments: [
                                        JavTemplateArgument::Type(
                                            JavType::PathLeading(
                                                JavTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                },
                                            ),
                                        ),
                                    ],
                                },
                            ),
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::pop_with_largest_opt_f32`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                template_arguments: [],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::collect_leashes`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::collect_leashes`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::start`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::start`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::cyclic_slice_leashed`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::first`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::first`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfPlace,
                                ),
                                JavTermSymbolResolution::SelfPlace,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::pop`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::pop`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<#derive _ as core::clone::Clone(0)>::clone`,
                            TraitItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfType,
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::pop`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::pop`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                JavTemplateArgument::Type(
                                    JavType::PathLeading(
                                        JavTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                JavTemplateArgument::Type(
                                                    JavType::PathLeading(
                                                        JavTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            template_arguments: [],
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                    ),
                                ),
                            ],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<#derive _ as core::clone::Clone(0)>::clone`,
                            TraitItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfType,
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::Type(
                        TypePath(`core::ops::ControlFlow`, `Enum`),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::ops::ControlFlow`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                template_arguments: [],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: None,
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                template_arguments: [
                                                                    JavTemplateArgument::Type(
                                                                        JavType::PathLeading(
                                                                            JavTypePathLeading {
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                template_arguments: [],
                                                                            },
                                                                        ),
                                                                    ),
                                                                ],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                JavTemplateArgument::Type(
                                    JavType::PathLeading(
                                        JavTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                JavTemplateArgument::Type(
                                                    JavType::PathLeading(
                                                        JavTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            template_arguments: [],
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                    ),
                                ),
                            ],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::end`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::end`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::cyclic_slice_leashed`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::end`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::end`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::last`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::last`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfPlace,
                                ),
                                JavTermSymbolResolution::SelfPlace,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::last`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::last`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::last`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::last`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfPlace,
                                ),
                                JavTermSymbolResolution::SelfPlace,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::pop_with_largest_opt_f32`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                template_arguments: [],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                            TraitItemKind::AssocRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: Some(
                            2,
                        ),
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::Type(
                        TypePath(`core::option::Option`, `Enum`),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::option::Option`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: None,
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::Form(
                        MajorFormPath(`malamute::narrow_down`, `Ritchie(
                            Gn,
                        )`),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`malamute::narrow_down`),
                        context: JavTypeContext {
                            comptime_var_overrides: [
                                (
                                    MajorItem(
                                        Form(
                                            MajorFormPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    Type(
                                        PathLeading(
                                            JavTypePathLeading(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: None,
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                template_arguments: [],
                                                            },
                                                        ),
                                                    ),
                                                ],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::Ritchie(
                                            JavRitchieType {
                                                parameters: [
                                                    JavRitchieParameter {
                                                        contract: Pure,
                                                        parameter_ty: PathLeading(
                                                            JavTypePathLeading(
                                                                Id {
                                                                    value: 12,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: JavType::PathLeading(
                                                    JavTypePathLeading {
                                                        ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        template_arguments: [
                                                            JavTemplateArgument::Type(
                                                                JavType::PathLeading(
                                                                    JavTypePathLeading {
                                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                                        template_arguments: [],
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                ),
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::last`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::last`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::Type(
                        TypePath(`core::ops::ControlFlow`, `Enum`),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::ops::ControlFlow`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: None,
                    },
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `malamute`,
            data: PackagePathSource::Registry {
                registry_path: RegistryPath(
                    "../../../.corgi/../registry",
                ),
                version: Version {
                    major: 0,
                    minor: 1,
                    patch: 0,
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: "../../../examples/mnist-classifier",
            },
        },
    },
    package_valkyrie_javelins: [
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                            TraitItemKind::AssocRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: Some(
                            2,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<#derive _ as core::clone::Clone(0)>::clone`,
                            TraitItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfType,
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::collect_leashes`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::collect_leashes`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                JavTemplateArgument::Type(
                                    JavType::PathLeading(
                                        JavTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                JavTemplateArgument::Type(
                                                    JavType::PathLeading(
                                                        JavTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            template_arguments: [],
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                    ),
                                ),
                            ],
                        },
                    ),
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::pop_with_largest_opt_f32`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                template_arguments: [],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                template_arguments: [
                                                                    JavTemplateArgument::Type(
                                                                        JavType::PathLeading(
                                                                            JavTypePathLeading {
                                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                                template_arguments: [],
                                                                            },
                                                                        ),
                                                                    ),
                                                                ],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::last`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::last`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfPlace,
                                ),
                                JavTermSymbolResolution::SelfPlace,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::pop`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::pop`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::first`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::first`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<#derive _ as core::clone::Clone(0)>::clone`,
                            TraitItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfType,
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::last`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::last`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::last`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::last`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfPlace,
                                ),
                                JavTermSymbolResolution::SelfPlace,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::start`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::start`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::pop`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::pop`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::end`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::end`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::first`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::first`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfPlace,
                                ),
                                JavTermSymbolResolution::SelfPlace,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::cyclic_slice_leashed`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::cyclic_slice_leashed`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::first`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::first`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::start`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::start`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::end`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::end`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::slice::CyclicSlice(0)::last`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::slice::CyclicSlice(0)::last`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::collect_leashes`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::collect_leashes`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                JavTemplateArgument::Type(
                                    JavType::PathLeading(
                                        JavTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                JavTemplateArgument::Type(
                                                    JavType::PathLeading(
                                                        JavTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            template_arguments: [],
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                    ),
                                ),
                            ],
                        },
                    ),
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::Ritchie(
                                            JavRitchieType {
                                                parameters: [
                                                    JavRitchieParameter {
                                                        contract: Pure,
                                                        parameter_ty: PathLeading(
                                                            JavTypePathLeading(
                                                                Id {
                                                                    value: 12,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: JavType::PathLeading(
                                                    JavTypePathLeading {
                                                        ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        template_arguments: [
                                                            JavTemplateArgument::Type(
                                                                JavType::PathLeading(
                                                                    JavTypePathLeading {
                                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                                        template_arguments: [],
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                ),
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::pop_with_largest_opt_f32`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                template_arguments: [],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::push`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::push`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                template_arguments: [
                                                                    JavTemplateArgument::Type(
                                                                        JavType::PathLeading(
                                                                            JavTypePathLeading {
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                template_arguments: [],
                                                                            },
                                                                        ),
                                                                    ),
                                                                ],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::SelfLifetime,
                                ),
                                JavTermSymbolResolution::SelfLifetime,
                            ),
                        ],
                        separator: Some(
                            1,
                        ),
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                template_arguments: [],
                                                            },
                                                        ),
                                                    ),
                                                ],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::Ritchie(
                        JavRitchieType {
                            parameters: [
                                JavRitchieParameter {
                                    contract: Pure,
                                    parameter_ty: PathLeading(
                                        JavTypePathLeading(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                },
                            ],
                            return_ty: JavType::PathLeading(
                                JavTypePathLeading {
                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                    template_arguments: [
                                        JavTemplateArgument::Type(
                                            JavType::PathLeading(
                                                JavTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                },
                                            ),
                                        ),
                                    ],
                                },
                            ),
                        },
                    ),
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::Form(
                        MajorFormPath(`malamute::narrow_down`, `Ritchie(
                            Gn,
                        )`),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`malamute::narrow_down`),
                        context: JavTypeContext {
                            comptime_var_overrides: [
                                (
                                    MajorItem(
                                        Form(
                                            MajorFormPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    Type(
                                        PathLeading(
                                            JavTypePathLeading(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: None,
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::TypeDefault {
                    ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::TypeItem(
                        TypeItemPath(
                            `core::vec::Vec(0)::ilen`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::vec::Vec(0)::ilen`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                template_arguments: [],
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
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::Type(
                        TypePath(`core::option::Option`, `Enum`),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::option::Option`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: None,
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::Type(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`malamute::Class`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: None,
                    },
                },
            },
        ),
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavPath::Type(
                        TypePath(`core::ops::ControlFlow`, `Enum`),
                    ),
                    instantiation: JavInstantiation {
                        path: ItemPath(`core::ops::ControlFlow`),
                        context: JavTypeContext {
                            comptime_var_overrides: [],
                        },
                        variable_resolutions: [
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
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                                template_arguments: [
                                                    JavTemplateArgument::Type(
                                                        JavType::PathLeading(
                                                            JavTypePathLeading {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                template_arguments: [],
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
                                        JavType::PathLeading(
                                            JavTypePathLeading {
                                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        separator: None,
                    },
                },
            },
        ),
    ],
}
```