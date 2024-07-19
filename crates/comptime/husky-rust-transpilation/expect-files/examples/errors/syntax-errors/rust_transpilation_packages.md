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
                    name: `syntax-errors`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/errors/syntax-errors",
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
            name: `syntax-errors`,
            data: PackagePathSource::Local {
                path: "../../../examples/errors/syntax-errors",
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
                    name: `syntax-errors`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/errors/syntax-errors",
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
            name: `syntax-errors`,
            data: PackagePathSource::Local {
                path: "../../../examples/errors/syntax-errors",
            },
        },
        kind: Linkets,
    },
]
```