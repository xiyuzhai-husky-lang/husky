```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Extern(
                ExternTypeHirDecl {
                    path: TypePath(`mnist::task::MnistTask`, `Extern`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist::task::MnistTask`, `Extern`),
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
    HirDecl::Attr(
        AttrHirDecl::Task(
            TaskAttrHirDecl {
                path: AttrItemPath(`mnist::task::MnistTask::@task(0)`),
            },
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath(`mnist::task::MnistTask(0)`),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist::task::MnistTask`, `Extern`),
                        template_arguments: [],
                        always_copyable: false,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::ItemDecl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TypeImplBlock(
                                TypeImplBlockPath(`mnist::task::MnistTask(0)`),
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::AssocRitchie(
                TypeAssocRitchieHirDecl {
                    path: TypeItemPath(
                        `mnist::task::MnistTask(0)::new`,
                        TypeItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist::task::MnistTask`, `Extern`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist::task::MnistTask(0)::new`,
                                        TypeItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
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
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
]
```