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
                    path: JavPath::TypeVariantConstructor(
                        TypeVariantPath(
                            ItemPathId(
                                Id {
                                    value: 101,
                                },
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateSvarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
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
                                HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateSvarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
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
                    path: JavPath::TypeVariantConstructor(
                        TypeVariantPath(
                            ItemPathId(
                                Id {
                                    value: 100,
                                },
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateSvarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
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
                                HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateSvarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
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
                    path: JavPath::TypeVariantConstructor(
                        TypeVariantPath(
                            ItemPathId(
                                Id {
                                    value: 101,
                                },
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateSvarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
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
                                HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateSvarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
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
                    path: JavPath::TypeVariantConstructor(
                        TypeVariantPath(
                            ItemPathId(
                                Id {
                                    value: 100,
                                },
                            ),
                        ),
                    ),
                    instantiation: JavInstantiation {
                        symbol_resolutions: [
                            (
                                HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateSvarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
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
                                HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateSvarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                JavTermSymbolResolution::Explicit(
                                    JavTemplateArgument::Type(
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