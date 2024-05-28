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
                name: `quick-sort`,
                data: PackagePathSource::Local {
                    path: VirtualPath {
                        _data: VirtualPathBuf(
                            "../../../examples/algorithms/quick-sort",
                        ),
                    },
                },
            },
            kind: Lib,
        },
        task_crate_path: None,
    },
)
```