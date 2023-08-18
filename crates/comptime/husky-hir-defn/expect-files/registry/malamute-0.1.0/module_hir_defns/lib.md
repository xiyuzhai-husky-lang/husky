[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumTypeHirDefn {
                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                                HirTemplateParameter {
                                    symbol: Const(
                                        HirConstSymbol(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                        hir_expr_region: HirEagerExprRegion,
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumTypeHirDefn {
                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                                HirTemplateParameter {
                                    symbol: Const(
                                        HirConstSymbol(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                        hir_expr_region: HirEagerExprRegion,
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Gn(
                GnHirDefn {
                    path: FugitivePath(`malamute::narrow_down`, `Gn`),
                    hir_decl: GnFugitiveHirDecl {
                        path: FugitivePath(`malamute::narrow_down`, `Gn`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                                HirTemplateParameter {
                                    symbol: Const(
                                        HirConstSymbol(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                    },
                    body: None,
                    hir_expr_region: HirLazyExprRegion,
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `malamute`,
                    trai_path: TraitPath(`core::ops::Unveil`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [
                        HirTemplateParameter {
                            symbol: Type(
                                Type {
                                    attrs: HirSymbolAttrs,
                                    variance: None,
                                    disambiguator: 0,
                                },
                            ),
                            traits: [],
                        },
                        HirTemplateParameter {
                            symbol: Const(
                                HirConstSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                            traits: [],
                        },
                    ],
                },
                trai: HirTrait {
                    trai_path: TraitPath(`core::ops::Unveil`),
                    template_arguments: [
                        Type(
                            PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 67,
                                    },
                                ),
                            ),
                        ),
                    ],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 66,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssociatedType(
                TraitForTypeAssociatedTypeHirDefn {
                    path: TraitForTypeItemPath {
                        impl_block: TraitForTypeImplBlockPath {
                            module_path: `malamute`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `Output`,
                        item_kind: AssociatedType,
                    },
                    hir_decl: TraitForTypeAssociatedTypeHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `malamute`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `Output`,
                            item_kind: AssociatedType,
                        },
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        associated_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 20,
                                },
                            ),
                        ),
                    },
                },
            ),
        ),
    ),
]