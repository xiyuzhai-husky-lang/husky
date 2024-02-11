[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructTypeHirDecl {
                    path: TypePath(`syntax_errors::ast::A`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [],
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`syntax_errors::ast::A`, `Struct`),
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
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `syntax_errors::ast`,
                                    ty_path: TypePath(`syntax_errors::ast::A`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`syntax_errors::ast::A`, `Struct`),
                        template_arguments: [],
                        always_copyable: false,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TypeImplBlock(
                                TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `syntax_errors::ast`,
                                                    ty_path: TypePath(`syntax_errors::ast::A`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    },
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
]