```rust
[
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(
                ItemPathId(
                    Id {
                        value: 1,
                    },
                ),
            ),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(
                ItemPathId(
                    Id {
                        value: 2,
                    },
                ),
            ),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(
                ItemPathId(
                    Id {
                        value: 3,
                    },
                ),
            ),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(
                ItemPathId(
                    Id {
                        value: 4,
                    },
                ),
            ),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(
                ItemPathId(
                    Id {
                        value: 5,
                    },
                ),
            ),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(
                ItemPathId(
                    Id {
                        value: 6,
                    },
                ),
            ),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(
                ItemPathId(
                    Id {
                        value: 7,
                    },
                ),
            ),
        },
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Val(
                MajorValHirDecl {
                    path: FormPath(`mnist_classifier::main`, `Val`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`malamute::Class`, `Enum`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::MajorItem(
                                MajorItemPath::Form(
                                    FormPath(`mnist_classifier::main`, `Val`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
]
```