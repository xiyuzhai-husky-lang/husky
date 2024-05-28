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
                    name: `ml-task`,
                    data: PackagePathSource::Local {
                        path: "../../../registry/ml-task",
                    },
                },
            ),
        },
        package_path: PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `ml-task`,
            data: PackagePathSource::Local {
                path: "../../../registry/ml-task",
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
                            library_path: "../../../library",
                        },
                    },
                    name: `ml-task`,
                    data: PackagePathSource::Local {
                        path: "../../../registry/ml-task",
                    },
                },
            ),
        },
        package_path: PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `ml-task`,
            data: PackagePathSource::Local {
                path: "../../../registry/ml-task",
            },
        },
        kind: Linkages,
    },
]
```