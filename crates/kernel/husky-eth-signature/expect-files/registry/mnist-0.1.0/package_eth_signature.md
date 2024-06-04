```rust
Ok(
    PackageEthSignature {
        path: PackagePath {
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
        data: PackageEthSignatureData {
            task_type: Some(
                ItemPath(
                    TypeOntology(
                        TypePath(
                            ItemPathId(
                                Id {
                                    value: 164,
                                },
                            ),
                        ),
                    ),
                ),
            ),
        },
    },
)
```