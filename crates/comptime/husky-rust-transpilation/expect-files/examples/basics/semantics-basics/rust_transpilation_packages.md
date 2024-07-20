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
                    name: `semantics-basics`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/basics/semantics-basics",
                    },
                },
            ),
        },
        data: Linkets,
    },
]
```