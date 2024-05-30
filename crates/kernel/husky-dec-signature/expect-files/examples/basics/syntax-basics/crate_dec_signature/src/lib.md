```rust
Ok(
    CrateDecSignature::Lib(
        LibCrateDecSignature {
            crate_path: CratePath {
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
                kind: Lib,
            },
        },
    ),
)
```