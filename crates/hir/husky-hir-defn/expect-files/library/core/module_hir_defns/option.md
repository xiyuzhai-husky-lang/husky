[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumTypeHirDefn {
                    path: TypePath(`core::option::Option`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`core::option::Option`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 115,
                                                },
                                            ),
                                        ),
                                        traits: [],
                                    },
                                },
                            ],
                        ),
                        hir_expr_region: HirEagerExprRegion {
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
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: Ident(
                                                Ident(
                                                    Coword(
                                                        Id {
                                                            value: 115,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                    ],
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
                },
            ),
        ),
    ),
]