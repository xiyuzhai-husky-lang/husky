```rust
Ok(
    PackageSynopsis::Lib {
        lib_crate_path: CratePath {
            package_path: PackagePath {
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
            kind: Lib,
        },
        task_crate_path: None,
    },
)
```