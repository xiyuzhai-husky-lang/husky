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
                name: `malamute`,
                data: PackagePathSource::Local {
                    path: "../../../registry/malamute-0.1.0",
                },
            },
            kind: Lib,
        },
        task_crate_path: None,
    },
)
```