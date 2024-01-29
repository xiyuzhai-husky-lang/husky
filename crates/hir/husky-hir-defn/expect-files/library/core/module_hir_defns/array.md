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
                                    symbol: HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::usize`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            index: HirConstSymbolIndex::PathLeading {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Comptime,
                                                },
                                                disambiguator: 0,
                                                ty_path: TypePath(`core::num::usize`, `Extern`),
                                            },
                                        },
                                    ),
                                    data: HirTemplateParameterData::Constant {
                                        ident: `L`,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::usize`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                },
                                HirTemplateParameter {
                                    symbol: HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: Some(
                                                Covariant,
                                            ),
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: HirTemplateParameterData::Type {
                                        ident: `E`,
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
                                                            always_copyable: true,
                                                        },
                                                    ),
                                                    index: HirConstSymbolIndex::PathLeading {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        disambiguator: 0,
                                                        ty_path: TypePath(`core::num::usize`, `Extern`),
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
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
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