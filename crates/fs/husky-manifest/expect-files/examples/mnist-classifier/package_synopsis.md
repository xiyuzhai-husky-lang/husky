```rust
Ok(
    PackageSynopsis::Main {
        main_crate_path: CratePath {
            package_path: PackagePath {
                toolchain: Toolchain {
                    data: ToolchainData::Local {
                        library_path: "../../../library",
                    },
                },
                name: `mnist-classifier`,
                data: PackagePathSource::Local {
                    path: "../../../examples/mnist-classifier",
                },
            },
            kind: Main,
        },
        task_crate_path: CratePath {
            package_path: PackagePath {
                toolchain: Toolchain {
                    data: ToolchainData::Local {
                        library_path: "../../../library",
                    },
                },
                name: `mnist`,
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
            kind: Task,
        },
    },
)
```