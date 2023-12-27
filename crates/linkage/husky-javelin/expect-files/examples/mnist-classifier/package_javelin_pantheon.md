ValkyrieJavelinPantheon {
    package_path: PackagePath {
        toolchain: Toolchain {
            data: ToolchainData::Local {
                library_path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../library",
                    ),
                },
            },
        },
        name: `mnist-classifier`,
        data: PackagePathSource::Local {
            path: VirtualPath {
                _data: VirtualPathBuf(
                    "../../../examples/mnist-classifier",
                ),
            },
        },
    },
    instantiation_map: {
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
                                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                template_arguments: [
                                                                    JavelinTemplateArgument::Type(
                                                                        JavelinType::PathLeading(
                                                                            JavelinTypePathLeading {
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
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                JavelinTemplateArgument::Type(
                                    JavelinType::PathLeading(
                                        JavelinTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                JavelinTemplateArgument::Type(
                                                    JavelinType::PathLeading(
                                                        JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfPlace,
                                ),
                                JavelinTermSymbolResolution::SelfPlace,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfPlace,
                                ),
                                JavelinTermSymbolResolution::SelfPlace,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
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
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::Class`, `Enum`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `unveil`,
                                            item_kind: AssociatedFunctionFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::Fugitive(
                        FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::Ritchie(
                                            JavelinRitchieType {
                                                parameters: [
                                                    JavelinRitchieParameter {
                                                        contract: Pure,
                                                        parameter_ty: PathLeading(
                                                            JavelinTypePathLeading(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: JavelinType::PathLeading(
                                                    JavelinTypePathLeading {
                                                        ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        template_arguments: [
                                                            JavelinTemplateArgument::Type(
                                                                JavelinType::PathLeading(
                                                                    JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
                                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                template_arguments: [
                                                                    JavelinTemplateArgument::Type(
                                                                        JavelinType::PathLeading(
                                                                            JavelinTypePathLeading {
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
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::TypeDefault {
                    ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::Ritchie(
                        JavelinRitchieType {
                            parameters: [
                                JavelinRitchieParameter {
                                    contract: Pure,
                                    parameter_ty: PathLeading(
                                        JavelinTypePathLeading(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                },
                            ],
                            return_ty: JavelinType::PathLeading(
                                JavelinTypePathLeading {
                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                    template_arguments: [
                                        JavelinTemplateArgument::Type(
                                            JavelinType::PathLeading(
                                                JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
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
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::first`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfPlace,
                                ),
                                JavelinTermSymbolResolution::SelfPlace,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `core::clone`,
                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                    ty_sketch: TypeSketch::DeriveAny,
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfType,
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
            },
        ): PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                JavelinTemplateArgument::Type(
                                    JavelinType::PathLeading(
                                        JavelinTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                JavelinTemplateArgument::Type(
                                                    JavelinType::PathLeading(
                                                        JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `core::clone`,
                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                    ty_sketch: TypeSketch::DeriveAny,
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfType,
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `mnist-classifier`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/mnist-classifier",
                    ),
                },
            },
        },
    },
    package_valkyrie_javelins: [
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::Class`, `Enum`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `unveil`,
                                            item_kind: AssociatedFunctionFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
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
                    path: JavelinPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `core::clone`,
                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                    ty_sketch: TypeSketch::DeriveAny,
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfType,
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                JavelinTemplateArgument::Type(
                                    JavelinType::PathLeading(
                                        JavelinTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                JavelinTemplateArgument::Type(
                                                    JavelinType::PathLeading(
                                                        JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
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
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
                                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                template_arguments: [
                                                                    JavelinTemplateArgument::Type(
                                                                        JavelinType::PathLeading(
                                                                            JavelinTypePathLeading {
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
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
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
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfPlace,
                                ),
                                JavelinTermSymbolResolution::SelfPlace,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `core::clone`,
                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                    ty_sketch: TypeSketch::DeriveAny,
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfType,
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfPlace,
                                ),
                                JavelinTermSymbolResolution::SelfPlace,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::first`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfPlace,
                                ),
                                JavelinTermSymbolResolution::SelfPlace,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                JavelinTemplateArgument::Type(
                                    JavelinType::PathLeading(
                                        JavelinTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                JavelinTemplateArgument::Type(
                                                    JavelinType::PathLeading(
                                                        JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::Ritchie(
                                            JavelinRitchieType {
                                                parameters: [
                                                    JavelinRitchieParameter {
                                                        contract: Pure,
                                                        parameter_ty: PathLeading(
                                                            JavelinTypePathLeading(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: JavelinType::PathLeading(
                                                    JavelinTypePathLeading {
                                                        ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        template_arguments: [
                                                            JavelinTemplateArgument::Type(
                                                                JavelinType::PathLeading(
                                                                    JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
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
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
                                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                template_arguments: [
                                                                    JavelinTemplateArgument::Type(
                                                                        JavelinType::PathLeading(
                                                                            JavelinTypePathLeading {
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
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::SelfLifetime,
                                ),
                                JavelinTermSymbolResolution::SelfLifetime,
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    JavelinTemplateArgument::Type(
                                                        JavelinType::PathLeading(
                                                            JavelinTypePathLeading {
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
                    element_ty: JavelinType::Ritchie(
                        JavelinRitchieType {
                            parameters: [
                                JavelinRitchieParameter {
                                    contract: Pure,
                                    parameter_ty: PathLeading(
                                        JavelinTypePathLeading(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                },
                            ],
                            return_ty: JavelinType::PathLeading(
                                JavelinTypePathLeading {
                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                    template_arguments: [
                                        JavelinTemplateArgument::Type(
                                            JavelinType::PathLeading(
                                                JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    path: JavelinPath::Fugitive(
                        FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
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
                    path: JavelinPath::TypeItem(
                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                    ),
                    instantiation: JavelinInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
    ],
}