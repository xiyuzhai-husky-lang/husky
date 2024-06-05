```rust
Ok(
    PackageEthSignature {
        path: PackagePath {
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
        data: PackageEthSignatureData {
            task_ty: None,
        },
    },
)
```