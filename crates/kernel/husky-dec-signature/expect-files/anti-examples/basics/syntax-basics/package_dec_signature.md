```rust
Ok(
    PackageDecSignature {
        path: PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `syntax-basics`,
            data: PackagePathSource::Local {
                path: "../../../examples/basics/syntax-basics",
            },
        },
        data: PackageDecSignatureData {
            task_ty_default: None,
        },
    },
)
```