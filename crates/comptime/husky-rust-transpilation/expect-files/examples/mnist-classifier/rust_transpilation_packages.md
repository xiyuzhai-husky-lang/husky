```rust
[
    RustTranspilationPackage {
        target_path: LinktimeTargetPath {
            data: LinktimeTargetPathData::Package(
                PackagePath {
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
            ),
        },
        data: RustTranspilationPackageData::Source {
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
        },
    },
    RustTranspilationPackage {
        target_path: LinktimeTargetPath {
            data: LinktimeTargetPathData::Package(
                PackagePath {
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
            ),
        },
        data: RustTranspilationPackageData::Source {
            package_path: PackagePath {
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
        },
    },
    RustTranspilationPackage {
        target_path: LinktimeTargetPath {
            data: LinktimeTargetPathData::Package(
                PackagePath {
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
            ),
        },
        data: RustTranspilationPackageData::Source {
            package_path: PackagePath {
                toolchain: Toolchain {
                    data: ToolchainData::Local {
                        library_path: "../../../library",
                    },
                },
                name: `mnist`,
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
        },
    },
    RustTranspilationPackage {
        target_path: LinktimeTargetPath {
            data: LinktimeTargetPathData::Package(
                PackagePath {
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
            ),
        },
        data: RustTranspilationPackageData::Source {
            package_path: PackagePath {
                toolchain: Toolchain {
                    data: ToolchainData::Local {
                        library_path: "../../../library",
                    },
                },
                name: `ml-task`,
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
        },
    },
    RustTranspilationPackage {
        target_path: LinktimeTargetPath {
            data: LinktimeTargetPathData::Package(
                PackagePath {
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
            ),
        },
        data: RustTranspilationPackageData::Linkets,
    },
]
```