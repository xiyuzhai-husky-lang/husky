```rust
Ok(
    [
        PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `ml-task`,
            data: PackagePathSource::Local {
                path: "../../../registry/ml-task-0.1.0",
            },
        },
        PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `core`,
            data: PackagePathSource::Library,
        },
    ],
)
```