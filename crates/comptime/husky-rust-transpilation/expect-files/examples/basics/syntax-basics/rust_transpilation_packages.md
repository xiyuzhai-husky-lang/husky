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
                    name: `syntax-basics`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/basics/syntax-basics",
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
            name: `syntax-basics`,
            data: PackagePathSource::Local {
                path: "../../../examples/basics/syntax-basics",
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
                    name: `syntax-basics`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/basics/syntax-basics",
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
            name: `syntax-basics`,
            data: PackagePathSource::Local {
                path: "../../../examples/basics/syntax-basics",
            },
        },
        kind: Linkets,
    },
]
```