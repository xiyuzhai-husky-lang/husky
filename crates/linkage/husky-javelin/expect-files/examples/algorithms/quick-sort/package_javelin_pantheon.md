```rust
ValkyrieJavelinPantheon {
    package_path: PackagePath {
        toolchain: Toolchain {
            data: ToolchainData::Local {
                library_path: "../../../library",
            },
        },
        name: `quick-sort`,
        data: PackagePathSource::Local {
            path: "../../../examples/algorithms/quick-sort",
        },
    },
    instantiation_map: {
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`core::mem::Ref`, `Extern`),
                            template_arguments: [
                                JavTemplateArgument::Constant(
                                    JavelinConstant::StaticLifetime,
                                ),
                                JavTemplateArgument::Type(
                                    JavType::PathLeading(
                                        JavTypePathLeading {
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
                    library_path: "../../../library",
                },
            },
            name: `quick-sort`,
            data: PackagePathSource::Local {
                path: "../../../examples/algorithms/quick-sort",
            },
        },
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`core::num::i32`, `Extern`),
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
            name: `quick-sort`,
            data: PackagePathSource::Local {
                path: "../../../examples/algorithms/quick-sort",
            },
        },
    },
    package_valkyrie_javelins: [
        ValkyrieJavelin(
            Javelin {
                data: JavelinData::VecConstructor {
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
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
                    element_ty: JavType::PathLeading(
                        JavTypePathLeading {
                            ty_path: TypePath(`core::mem::Ref`, `Extern`),
                            template_arguments: [
                                JavTemplateArgument::Constant(
                                    JavelinConstant::StaticLifetime,
                                ),
                                JavTemplateArgument::Type(
                                    JavType::PathLeading(
                                        JavTypePathLeading {
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
```