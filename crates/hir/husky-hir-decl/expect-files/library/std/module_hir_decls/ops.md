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
                                            value: 179,
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
                    runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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