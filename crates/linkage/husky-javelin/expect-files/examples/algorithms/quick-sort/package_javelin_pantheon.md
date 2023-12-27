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
        name: `quick-sort`,
        data: PackagePathSource::Local {
            path: VirtualPath {
                _data: VirtualPathBuf(
                    "../../../examples/algorithms/quick-sort",
                ),
            },
        },
    },
    instantiation_map: {
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`core::num::i32`, `Extern`),
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
            name: `quick-sort`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/algorithms/quick-sort",
                    ),
                },
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`core::mem::Ref`, `Extern`),
                            template_arguments: [
                                JavelinTemplateArgument::Constant(
                                    JavelinConstant::StaticLifetime,
                                ),
                                JavelinTemplateArgument::Type(
                                    JavelinType::PathLeading(
                                        JavelinTypePathLeading {
                                            ty_path: TypePath(`core::str::str`, `Extern`),
                                            template_arguments: [],
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
            name: `quick-sort`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../examples/algorithms/quick-sort",
                    ),
                },
            },
        },
    },
    package_valkyrie_javelins: [
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavelinType::PathLeading(
                        JavelinTypePathLeading {
                            ty_path: TypePath(`core::num::i32`, `Extern`),
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
                            ty_path: TypePath(`core::mem::Ref`, `Extern`),
                            template_arguments: [
                                JavelinTemplateArgument::Constant(
                                    JavelinConstant::StaticLifetime,
                                ),
                                JavelinTemplateArgument::Type(
                                    JavelinType::PathLeading(
                                        JavelinTypePathLeading {
                                            ty_path: TypePath(`core::str::str`, `Extern`),
                                            template_arguments: [],
                                        },
                                    ),
                                ),
                            ],
                        },
                    ),
                },
            },
        ),
    ],
}