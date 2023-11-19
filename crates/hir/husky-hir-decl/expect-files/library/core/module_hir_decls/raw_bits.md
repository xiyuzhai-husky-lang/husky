[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 28,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `core::raw_bits`,
                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                        template_arguments: [],
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TypeImplBlock(
                                TypeImplBlockPath {
                                    module_path: `core::raw_bits`,
                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    ),
                    hir_eager_expr_arena: Arena {
                        data: [],
                    },
                    hir_eager_stmt_arena: Arena {
                        data: [],
                    },
                    hir_eager_pattern_expr_arena: Arena {
                        data: [],
                    },
                    hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                        arena: Arena {
                            data: [],
                        },
                    },
                    hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                        arena: Arena {
                            data: [],
                        },
                        self_value_variable: None,
                    },
                },
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 27,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 28,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 29,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 30,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 31,
                    },
                ),
            ),
        ),
    ),
]