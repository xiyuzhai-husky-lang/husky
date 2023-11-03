[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumTypeHirDefn {
                    path: TypePath(`malamute::Class`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`malamute::Class`, `Enum`),
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
                            ],
                        },
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
                        },
                    },
                },
            ),
        ),
    ),
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
                        },
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
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionGn(
                FunctionGnHirDefn {
                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                    hir_decl: GnFugitiveHirDecl {
                        path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
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
                    lazy_body_with_hir_lazy_expr_region: None,
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::Class`, `Enum`),
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
                                            value: 69,
                                        },
                                    ),
                                ),
                            ),
                        ],
                    },
                    self_ty: PathLeading(
                        HirTypePathLeading(
                            Id {
                                value: 68,
                            },
                        ),
                    ),
                },
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
                                TypePath(`malamute::Class`, `Enum`),
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
                                    TypePath(`malamute::Class`, `Enum`),
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
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
                                            value: 70,
                                        },
                                    ),
                                ),
                            ),
                        ],
                    },
                    self_ty: PathLeading(
                        HirTypePathLeading(
                            Id {
                                value: 69,
                            },
                        ),
                    ),
                },
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