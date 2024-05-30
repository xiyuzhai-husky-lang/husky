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
                    name: `quick-sort`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/algorithms/quick-sort",
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
            name: `quick-sort`,
            data: PackagePathSource::Local {
                path: "../../../examples/algorithms/quick-sort",
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
                    name: `quick-sort`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/algorithms/quick-sort",
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
            name: `quick-sort`,
            data: PackagePathSource::Local {
                path: "../../../examples/algorithms/quick-sort",
            },
        },
        kind: Linkages,
    },
]
```