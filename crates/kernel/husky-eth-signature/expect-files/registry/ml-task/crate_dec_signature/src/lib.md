```rust
Ok(
    CrateEthSignature::Lib(
        LibCrateEthSignature {
            crate_path: CratePath {
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
                kind: Lib,
            },
        },
    ),
)
```