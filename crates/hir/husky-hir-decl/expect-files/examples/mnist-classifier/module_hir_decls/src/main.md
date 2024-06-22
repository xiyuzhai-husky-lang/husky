```rust
[
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(`mnist_classifier::connected_component),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(`mnist_classifier::raw_contour),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(`mnist_classifier::geom2d),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(`mnist_classifier::fermi),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(`mnist_classifier::digits),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmoduleItemPath(`mnist_classifier::major),
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
                            ItemPath(`mnist_classifier::main`),
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