[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Enum(
                EnumTypeHirDecl {
                    path: TypePath(`core::option::Option`, `Enum`),
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateVarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `T`,
                                    traits: [],
                                },
                            },
                        ],
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::option::Option`, `Enum`),
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
                                data: [
                                    HirEagerComptimeSvarEntry {
                                        name: HirEagerComptimeSvarName::Ident(
                                            `T`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateVar::Type(
                                            HirTypeSvar::Type {
                                                attrs: HirTemplateVarAttrs {
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                ],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRvarRegionData {
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