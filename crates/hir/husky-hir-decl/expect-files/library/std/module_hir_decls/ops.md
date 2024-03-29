```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Trait(
            TraitHirDecl {
                path: TraitPath(`std::ops::Add`),
                template_parameters: HirTemplateParameters(
                    [
                        HirTemplateParameter {
                            symbol: HirTemplateSvar::Type(
                                HirTypeSvar::Type {
                                    attrs: HirTemplateSvarAttrs {
                                        class: Comptime,
                                    },
                                    variance: None,
                                    disambiguator: 0,
                                },
                            ),
                            data: HirTemplateParameterData::Type {
                                ident: `B`,
                                traits: [],
                            },
                        },
                    ],
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::Decl(
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
                    pattern_arena: Arena {
                        data: [],
                    },
                    comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                        arena: Arena {
                            data: [
                                HirEagerComptimeSvarEntry {
                                    name: HirEagerComptimeSvarName::Ident(
                                        `B`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirTemplateSvar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateSvarAttrs {
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
```