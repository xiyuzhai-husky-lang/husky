```rust
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
                    name: `mnist`,
                    data: PackagePathSource::Local {
                        path: VirtualPath {
                            _data: VirtualPathBuf(
                                "../../../registry/mnist-0.1.0",
                            ),
                        },
                    },
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
            name: `mnist`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../registry/mnist-0.1.0",
                    ),
                },
            },
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
                    name: `mnist`,
                    data: PackagePathSource::Local {
                        path: VirtualPath {
                            _data: VirtualPathBuf(
                                "../../../registry/mnist-0.1.0",
                            ),
                        },
                    },
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
            name: `mnist`,
            data: PackagePathSource::Local {
                path: VirtualPath {
                    _data: VirtualPathBuf(
                        "../../../registry/mnist-0.1.0",
                    ),
                },
            },
        },
        kind: Linkages,
    },
]
```