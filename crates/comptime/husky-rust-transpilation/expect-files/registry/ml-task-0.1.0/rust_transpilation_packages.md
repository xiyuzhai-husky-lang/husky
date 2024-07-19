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
                        path: "../../../registry/ml-task-0.1.0",
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
                path: "../../../registry/ml-task-0.1.0",
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
                        path: "../../../registry/ml-task-0.1.0",
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
                path: "../../../registry/ml-task-0.1.0",
            },
        },
        kind: Linkets,
    },
]
```