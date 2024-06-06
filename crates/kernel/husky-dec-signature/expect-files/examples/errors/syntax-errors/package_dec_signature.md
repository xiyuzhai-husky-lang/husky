```rust
Ok(
    PackageDecSignature {
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
        data: PackageDecSignatureData {
            task_ty_default: None,
        },
    },
)
```