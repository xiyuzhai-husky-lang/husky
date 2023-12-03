[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternHirDefn {
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
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::array::Array`, `Extern`),
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
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `L`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateSymbol::Const(
                                                HirConstSymbol {
                                                    ty: HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`core::num::usize`, `Extern`),
                                                            template_arguments: [],
                                                        },
                                                    ),
                                                    index: PathLeading {
                                                        attrs: HirSymbolAttrs,
                                                        disambiguator: 0,
                                                        ty_path: TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 51,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `E`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateSymbol::Type(
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
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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