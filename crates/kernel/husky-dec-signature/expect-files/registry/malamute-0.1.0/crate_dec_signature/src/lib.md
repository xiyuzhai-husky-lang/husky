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
                    name: `malamute`,
                    data: PackagePathSource::Local {
                        path: "../../../registry/malamute-0.1.0",
                    },
                },
                kind: Lib,
            },
            default_const_excludes: Some(
                [
                    DecTerm::TypeAsTraitItem(
                        DecTypeAsTraitItem {
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Form(
                                    MajorFormPath(`core::task::Task`, `TypeVar`),
                                ),
                            ),
                            trai: DecTerm::ItemPath(
                                DecItemPath::Trait(
                                    TraitPath(`ml_task::IsMlTask`),
                                ),
                            ),
                            ident: `INPUT`,
                        },
                    ),
                ],
            ),
        },
    ),
)
```