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
                    name: `syntax-errors`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/errors/syntax-errors",
                    },
                },
                kind: Lib,
            },
        },
    ),
)
```