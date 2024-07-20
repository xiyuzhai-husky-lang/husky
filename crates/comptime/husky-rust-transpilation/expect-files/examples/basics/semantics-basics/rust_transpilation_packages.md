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
                    name: `semantics-basics`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/basics/semantics-basics",
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
                name: `semantics-basics`,
                data: PackagePathSource::Local {
                    path: "../../../examples/basics/semantics-basics",
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
                    name: `semantics-basics`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/basics/semantics-basics",
                    },
                },
            ),
        },
        data: RustTranspilationPackageData::Linkets,
    },
]
```