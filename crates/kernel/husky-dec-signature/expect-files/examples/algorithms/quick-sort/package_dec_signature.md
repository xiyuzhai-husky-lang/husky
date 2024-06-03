```rust
Ok(
    PackageDecSignature {
        path: PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `quick-sort`,
            data: PackagePathSource::Local {
                path: "../../../examples/algorithms/quick-sort",
            },
        },
        data: PackageDecSignatureData {
            task_ty_term: None,
        },
    },
)
```