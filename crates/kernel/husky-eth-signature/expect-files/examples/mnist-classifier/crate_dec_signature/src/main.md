```rust
Ok(
    CrateEthSignature::Main(
        MainCrateEthSignature {
            crate_path: CratePath {
                package_path: PackagePath {
                    toolchain: Toolchain {
                        data: ToolchainData::Local {
                            library_path: "../../../library",
                        },
                    },
                    name: `mnist-classifier`,
                    data: PackagePathSource::Local {
                        path: "../../../examples/mnist-classifier",
                    },
                },
                kind: Main,
            },
        },
    ),
)
```