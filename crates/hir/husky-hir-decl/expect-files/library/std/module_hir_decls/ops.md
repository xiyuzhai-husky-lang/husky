[
    HirDecl::MajorItem(
        MajorItemHirDecl::Trait(
            TraitHirDecl {
                path: TraitPath(`std::ops::Add`),
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
                                            value: 188,
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
                            MajorItemPath::Trait(
                                TraitPath(`std::ops::Add`),
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
                            data: [
                                HirEagerComptimeSymbolEntry {
                                    name: HirEagerComptimeSymbolName::Ident(
                                        `B`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirComptimeSymbol::Type(
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
        ),
    ),
]