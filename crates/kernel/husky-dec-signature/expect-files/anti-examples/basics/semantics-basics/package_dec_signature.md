```rust
Ok(
    PackageDecSignature {
        path: PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `semantics-basics`,
            data: PackagePathSource::Local {
                path: "../../../examples/basics/semantics-basics",
            },
        },
        data: PackageDecSignatureData {
            task_ty_default: None,
        },
    },
)
```