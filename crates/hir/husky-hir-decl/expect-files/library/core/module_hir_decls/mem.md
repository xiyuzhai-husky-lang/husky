[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Extern(
                ExternTypeHirDecl {
                    path: TypePath(`core::mem::Ref`, `Extern`),
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateSymbol::Lifetime(
                                    HirLifetimeSymbol {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: Some(
                                            Covariant,
                                        ),
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Lifetime {
                                    label: `'a`,
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
                                    TypePath(`core::mem::Ref`, `Extern`),
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
                                        name: HirEagerComptimeSymbolName::Label(
                                            `'a`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateSymbol::Lifetime(
                                            HirLifetimeSymbol {
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
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Extern(
                ExternTypeHirDecl {
                    path: TypePath(`core::mem::RefMut`, `Extern`),
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateSymbol::Lifetime(
                                    HirLifetimeSymbol {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: Some(
                                            Covariant,
                                        ),
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Lifetime {
                                    label: `'a`,
                                },
                            },
                            HirTemplateParameter {
                                symbol: HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: Some(
                                            Invariant,
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
                                    TypePath(`core::mem::RefMut`, `Extern`),
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
                                        name: HirEagerComptimeSymbolName::Label(
                                            `'a`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateSymbol::Lifetime(
                                            HirLifetimeSymbol {
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
                                                    Invariant,
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
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Extern(
                ExternTypeHirDecl {
                    path: TypePath(`core::mem::Leash`, `Extern`),
                    template_parameters: HirTemplateParameters(
                        [
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
                                    TypePath(`core::mem::Leash`, `Extern`),
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
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Extern(
                ExternTypeHirDecl {
                    path: TypePath(`core::mem::At`, `Extern`),
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateSymbol::Place(
                                    HirPlaceSymbol {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Place {
                                    label: `'α`,
                                },
                            },
                            HirTemplateParameter {
                                symbol: HirTemplateSymbol::Type(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
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
                                    TypePath(`core::mem::At`, `Extern`),
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
                                        name: HirEagerComptimeSymbolName::Label(
                                            `'α`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateSymbol::Place(
                                            HirPlaceSymbol {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 0,
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
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `core::mem`,
                        trai_path: TraitPath(`core::marker::Copy`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::mem::Leash`, `Extern`),
                        ),
                        disambiguator: 0,
                    },
                },
                template_parameters: HirTemplateParameters(
                    [
                        HirTemplateParameter {
                            symbol: HirTemplateSymbol::Type(
                                HirTypeSymbol::Type {
                                    attrs: HirTemplateSymbolAttrs {
                                        class: Comptime,
                                    },
                                    variance: None,
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
                trai: HirTrait {
                    trai_path: TraitPath(`core::marker::Copy`),
                    template_arguments: [],
                },
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                        template_arguments: [
                            HirTemplateArgument::Type(
                                HirType::Symbol(
                                    HirTypeSymbol::Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        ],
                        always_copyable: true,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TraitForTypeImplBlock(
                                TraitForTypeImplBlock {
                                    data: TraitForTypeImplBlockPathData {
                                        module_path: `core::mem`,
                                        trai_path: TraitPath(`core::marker::Copy`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`core::mem::Leash`, `Extern`),
                                        ),
                                        disambiguator: 0,
                                    },
                                },
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
                                        `E`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
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