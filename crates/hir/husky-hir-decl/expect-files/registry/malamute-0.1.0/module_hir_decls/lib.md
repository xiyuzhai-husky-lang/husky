[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Enum(
                EnumTypeHirDecl(
                    Id {
                        value: 5,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Enum(
                EnumTypeHirDecl(
                    Id {
                        value: 6,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Enum(
                EnumTypeHirDecl(
                    Id {
                        value: 7,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionGn(
                FunctionGnFugitiveHirDecl {
                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
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
                                                value: 545,
                                            },
                                        ),
                                    ),
                                    traits: [],
                                },
                            },
                        ],
                    ),
                    parenate_parameters: HirLazyParenateParameters(
                        [
                            Variadic {
                                variant: Vec,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                ),
                            },
                            Keyed {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 453,
                                        },
                                    ),
                                ),
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::Symbol(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                ),
                            ],
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion {
                        hir_lazy_expr_arena: Arena {
                            data: [],
                        },
                        hir_lazy_stmt_arena: Arena {
                            data: [],
                        },
                        hir_lazy_pattern_expr_arena: Arena {
                            data: [],
                        },
                        hir_lazy_variable_region: HirLazyVariableRegion {
                            arena: Arena {
                                data: [
                                    HirLazyVariable {
                                        name: Ident(
                                            Ident(
                                                Coword(
                                                    Id {
                                                        value: 154,
                                                    },
                                                ),
                                            ),
                                        ),
                                        data: ParenateParameter,
                                    },
                                    HirLazyVariable {
                                        name: Ident(
                                            Ident(
                                                Coword(
                                                    Id {
                                                        value: 453,
                                                    },
                                                ),
                                            ),
                                        ),
                                        data: ParenateParameter,
                                    },
                                ],
                            },
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
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::Class`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
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
                                            value: 545,
                                        },
                                    ),
                                ),
                                traits: [],
                            },
                        },
                    ],
                ),
                trai: HirTrait {
                    trai_path: TraitPath(`core::ops::Unveil`),
                    template_arguments: [
                        HirTemplateArgument::Type(
                            HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    template_arguments: [
                                        HirTemplateArgument::Type(
                                            HirType::Symbol(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    ],
                                },
                            ),
                        ),
                    ],
                },
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`malamute::Class`, `Enum`),
                        template_arguments: [
                            HirTemplateArgument::Type(
                                HirType::Symbol(
                                    Type {
                                        attrs: HirSymbolAttrs,
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        ],
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TraitForTypeImplBlock(
                                TraitForTypeImplBlock {
                                    data: TraitForTypeImplBlockPathData {
                                        module_path: `malamute`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`malamute::Class`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                },
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
                                        `Label`,
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
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            AssociatedType(
                TraitForTypeAssociatedTypeHirDecl(
                    Id {
                        value: 3,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::OneVsAll`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
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
                                            value: 545,
                                        },
                                    ),
                                ),
                                traits: [],
                            },
                        },
                    ],
                ),
                trai: HirTrait {
                    trai_path: TraitPath(`core::ops::Unveil`),
                    template_arguments: [
                        HirTemplateArgument::Type(
                            HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    template_arguments: [
                                        HirTemplateArgument::Type(
                                            HirType::Symbol(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    ],
                                },
                            ),
                        ),
                    ],
                },
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                        template_arguments: [
                            HirTemplateArgument::Type(
                                HirType::Symbol(
                                    Type {
                                        attrs: HirSymbolAttrs,
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        ],
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TraitForTypeImplBlock(
                                TraitForTypeImplBlock {
                                    data: TraitForTypeImplBlockPathData {
                                        module_path: `malamute`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                },
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
                                        `Label`,
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
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            AssociatedType(
                TraitForTypeAssociatedTypeHirDecl(
                    Id {
                        value: 4,
                    },
                ),
            ),
        ),
    ),
]