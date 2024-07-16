```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::StaticVar(
                MajorStaticVarHirDefn {
                    path: MajorFormPath(`mnist::TASK`, `StaticVar`),
                    hir_decl: MajorStaticVarHirDecl {
                        path: MajorFormPath(`mnist::TASK`, `StaticVar`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist::task::MnistTask`, `Extern`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist::TASK`),
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
                    hir_expr_body_and_region: None,
                },
            ),
        ),
    ),
]
```