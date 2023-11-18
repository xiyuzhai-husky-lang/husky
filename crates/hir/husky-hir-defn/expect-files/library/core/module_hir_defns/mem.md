[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::mem::Ref`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::mem::Ref`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Lifetime(
                                        HirLifetimeSymbol {
                                            attrs: HirSymbolAttrs,
                                            variance: Some(
                                                Covariant,
                                            ),
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Lifetime {
                                        label: Label {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                            kind: AllNonGreek,
                                        },
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
                                            name: Label(
                                                Label {
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 53,
                                                            },
                                                        ),
                                                    ),
                                                    kind: AllNonGreek,
                                                },
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: Lifetime(
                                                HirLifetimeSymbol {
                                                    attrs: HirSymbolAttrs,
                                                    variance: Some(
                                                        Covariant,
                                                    ),
                                                    disambiguator: 0,
                                                },
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::mem::RefMut`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::mem::RefMut`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Lifetime(
                                        HirLifetimeSymbol {
                                            attrs: HirSymbolAttrs,
                                            variance: Some(
                                                Covariant,
                                            ),
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Lifetime {
                                        label: Label {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                            kind: AllNonGreek,
                                        },
                                    },
                                },
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: Some(
                                                Invariant,
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
                                            name: Label(
                                                Label {
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 53,
                                                            },
                                                        ),
                                                    ),
                                                    kind: AllNonGreek,
                                                },
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: Lifetime(
                                                HirLifetimeSymbol {
                                                    attrs: HirSymbolAttrs,
                                                    variance: Some(
                                                        Covariant,
                                                    ),
                                                    disambiguator: 0,
                                                },
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
                                                        Invariant,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::mem::Leash`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::mem::Leash`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::mem::At`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::mem::At`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Place(
                                        HirPlaceSymbol {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Place {
                                        label: Label {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 57,
                                                    },
                                                ),
                                            ),
                                            kind: AllGreek,
                                        },
                                    },
                                },
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
                                            name: Label(
                                                Label {
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 57,
                                                            },
                                                        ),
                                                    ),
                                                    kind: AllGreek,
                                                },
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: Place(
                                                HirPlaceSymbol {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `core::mem`,
                        trai_path: TraitPath(`core::marker::Copy`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::mem::Leash`, `Extern`),
                        ),
                        disambiguator: 0,
                    },
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
                                                value: 25,
                                            },
                                        ),
                                    ),
                                    traits: [],
                                },
                            },
                        ],
                    ),
                },
            },
        ),
    ),
]