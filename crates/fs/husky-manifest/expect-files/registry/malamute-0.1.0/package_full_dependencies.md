```rust
Ok(
    [
        PackagePath {
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
        PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `core`,
            data: PackagePathSource::Library,
        },
        PackagePath {
            toolchain: Toolchain {
                data: ToolchainData::Local {
                    library_path: "../../../library",
                },
            },
            name: `ml-task`,
            data: PackagePathSource::Registry {
                registry_path: RegistryPath(
                    "../../../.corgi/../registry",
                ),
                version: Version {
                    major: 0,
                    minor: 1,
                    patch: 0,
                },
            },
        },
    ],
)
```