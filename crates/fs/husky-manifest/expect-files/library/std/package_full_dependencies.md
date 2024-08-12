```rust
Ok(
    [
        PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `std`,
            data: PackagePathSource::Library,
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