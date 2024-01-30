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
        name: `malamute`,
        data: PackagePathSource::Local {
            path: VirtualPath {
                _data: VirtualPathBuf(
                    "../../../registry/malamute-0.1.0",
                ),
            },
        },
    },
    instantiation_map: {
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeVariantConstructor(
                        TypeVariantPath(
                            ItemPathId {
                                data: ItemPathData::TypeVariant(
                                    TypeVariantPathData {
                                        parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ident: `Break`,
                                        index: U8(
                                            1,
                                        ),
                                    },
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
                                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `malamute`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../registry/malamute-0.1.0",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeVariantConstructor(
                        TypeVariantPath(
                            ItemPathId {
                                data: ItemPathData::TypeVariant(
                                    TypeVariantPathData {
                                        parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ident: `Continue`,
                                        index: U8(
                                            0,
                                        ),
                                    },
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
                                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
                    library_path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../library",
                        ),
                    },
                },
            },
            name: `malamute`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../registry/malamute-0.1.0",
                    ),
                },
            },
        },
    },
    package_valkyrie_javelins: [
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeVariantConstructor(
                        TypeVariantPath(
                            ItemPathId {
                                data: ItemPathData::TypeVariant(
                                    TypeVariantPathData {
                                        parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ident: `Break`,
                                        index: U8(
                                            1,
                                        ),
                                    },
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
                                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::PathLeading {
                    path: JavelinPath::TypeVariantConstructor(
                        TypeVariantPath(
                            ItemPathId {
                                data: ItemPathData::TypeVariant(
                                    TypeVariantPathData {
                                        parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ident: `Continue`,
                                        index: U8(
                                            0,
                                        ),
                                    },
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
                                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            (
                                HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavelinTermSymbolResolution::Explicit(
                                    JavelinTemplateArgument::Type(
                                        JavelinType::PathLeading(
                                            JavelinTypePathLeading {
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