```rust
Ok(
    PackageEthSignature {
        path: PackagePath {
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
        data: PackageEthSignatureData {
            task_ty: Some(
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