```rust
Ok(
    PackageDecSignature {
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
        data: PackageDecSignatureData {
            task_ty_default: Some(
                EntityPath(
                    Type(
                        TypePath(
                            ItemPathId(
                                Id {
                                    value: 163,
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