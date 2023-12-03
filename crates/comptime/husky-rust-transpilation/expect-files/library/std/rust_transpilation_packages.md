[
    RustTranspilationPackage {
        target_path: LinktimeTargetPath {
            data: LinktimeTargetPathData::Package(
                PackagePath {
                    toolchain: Toolchain {
                        data: ToolchainData::Local {
                            library_path: VirtualPath {
                                _data: VirtualPathBuf(
                                    "../../../library",
                                ),
                            },
                        },
                    },
                    name: `std`,
                    data: PackagePathSource::Library,
                },
            ),
        },
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
            name: `std`,
            data: PackagePathSource::Library,
        },
        kind: Source,
    },
    RustTranspilationPackage {
        target_path: LinktimeTargetPath {
            data: LinktimeTargetPathData::Package(
                PackagePath {
                    toolchain: Toolchain {
                        data: ToolchainData::Local {
                            library_path: VirtualPath {
                                _data: VirtualPathBuf(
                                    "../../../library",
                                ),
                            },
                        },
                    },
                    name: `std`,
                    data: PackagePathSource::Library,
                },
            ),
        },
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
            name: `std`,
            data: PackagePathSource::Library,
        },
        kind: Linkages,
    },
]