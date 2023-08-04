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
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Gn(
                GnFugitiveHirDecl {
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
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `malamute`,
                    trai_path: TraitPath(`core::ops::Unveil`),
                    ty_sketch: Path(
                        TypePath(
                            Id {
                                value: 104,
                            },
                        ),
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
                                        value: 65,
                                    },
                                ),
                            ),
                        ),
                    ],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 64,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            AssociatedType(
                TraitForTypeAssociatedTypeHirDecl(
                    Id {
                        value: 2,
                    },
                ),
            ),
        ),
    ),
]