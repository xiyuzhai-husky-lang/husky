```rust
Ok(
    PackageDecSignature {
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
        data: PackageDecSignatureData {
            task_ty_default: Some(
                ItemPath(
                    Type(
                        TypePath(
                            ItemPathId(
                                Id {
                                    value: 158,
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