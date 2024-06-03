```rust
Ok(
    PackageDecSignature {
        path: PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `malamute`,
            data: PackagePathSource::Local {
                path: "../../../registry/malamute-0.1.0",
            },
        },
        data: PackageDecSignatureData {
            task_ty_term: None,
        },
    },
)
```