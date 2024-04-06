```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Ki(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
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
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    hir_expr_body_and_region: Some(
                        (
                            Eager(
                                1,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
]
```