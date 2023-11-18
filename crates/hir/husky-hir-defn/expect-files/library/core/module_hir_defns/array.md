[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::array::Array`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::array::Array`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Const(
                                        HirConstSymbol(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                    data: Constant {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 23,
                                                },
                                            ),
                                        ),
                                        ty: PathLeading(
                                            HirTypePathLeading(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    },
                                },
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: Some(
                                                Covariant,
                                            ),
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 25,
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
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: Const(
                                                HirConstSymbol(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: Ident(
                                                Ident(
                                                    Coword(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: Some(
                                                        Covariant,
                                                    ),
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