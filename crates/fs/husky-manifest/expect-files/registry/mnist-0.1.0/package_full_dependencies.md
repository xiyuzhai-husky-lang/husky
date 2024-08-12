```rust
Ok(
    [
        PackagePath {
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