```rust
Ok(
    PackageSynopsis::Lib {
        lib_crate_path: CratePath {
            package_path: PackagePath {
                toolchain: Toolchain {
                    data: ToolchainData::Local {
                        library_path: VirtualPath {
                            _data: VirtualPathBuf(
                                "../../../library",
                            ),
                        },
                    },
                },
                name: `mnist`,
                data: PackagePathSource::Local {
                    path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../registry/mnist-0.1.0",
                        ),
                    },
                },
            },
            kind: Lib,
        },
        task_crate_path: Some(
            CratePath {
                package_path: PackagePath {
                    toolchain: Toolchain {
                        data: ToolchainData::Local {
                            library_path: VirtualPath {
                                _data: VirtualPathBuf(
                                    "../../../library",
                                ),
                            },
                        },
                    },
                    name: `mnist`,
                    data: PackagePathSource::Local {
                        path: VirtualPath {
                            _data: VirtualPathBuf(
                                "../../../registry/mnist-0.1.0",
                            ),
                        },
                    },
                },
                kind: Task,
            },
        ),
    },
)
```