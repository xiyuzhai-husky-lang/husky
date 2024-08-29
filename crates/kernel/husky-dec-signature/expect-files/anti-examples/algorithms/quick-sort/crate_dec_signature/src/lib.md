```rust
Ok(
    CrateDecSignature::Lib(
        LibCrateDecSignature {
            crate_path: CratePath {
                package_path: PackagePath {
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
                kind: Lib,
            },
            default_const_excludes: None,
        },
    ),
)
```