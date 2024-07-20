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
        data: Source {
            package_path: PackagePath(
                Id {
                    value: 1,
                },
            ),
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
                    name: `quick-sort`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/algorithms/quick-sort",
                    },
                },
            ),
        },
        data: Linkets,
    },
]
```