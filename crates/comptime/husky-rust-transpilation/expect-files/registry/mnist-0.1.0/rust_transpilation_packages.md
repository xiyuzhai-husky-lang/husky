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
                    name: `mnist`,
                    data: PackagePathSource::Local {
                        path: "../../../registry/mnist-0.1.0",
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
                    name: `mnist`,
                    data: PackagePathSource::Local {
                        path: "../../../registry/mnist-0.1.0",
                    },
                },
            ),
        },
        data: Linkets,
    },
]
```