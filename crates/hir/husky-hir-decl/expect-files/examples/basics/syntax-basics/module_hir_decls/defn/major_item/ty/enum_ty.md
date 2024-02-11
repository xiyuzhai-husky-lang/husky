[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Enum(
                EnumTypeHirDecl {
                    path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
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
            ),
        ),
    ),
]