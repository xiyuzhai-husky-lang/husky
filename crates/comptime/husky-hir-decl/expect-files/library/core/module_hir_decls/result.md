[
    HirDecl::ModuleItem(
        ModuleItemHirDecl::Type(
            Enum(
                EnumTypeHirDecl(
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
                path: TraitForTypeImplBlockPath {
                    module_path: `core::result`,
                    trai_path: TraitPath(`core::ops::Unveil`),
                    ty_sketch: Path(
                        TypePath(
                            Id {
                                value: 31,
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
                            symbol: Type(
                                Type {
                                    attrs: HirSymbolAttrs,
                                    variance: None,
                                    disambiguator: 1,
                                },
                            ),
                            traits: [],
                        },
                        HirTemplateParameter {
                            symbol: Type(
                                Type {
                                    attrs: HirSymbolAttrs,
                                    variance: None,
                                    disambiguator: 2,
                                },
                            ),
                            traits: [],
                        },
                        HirTemplateParameter {
                            symbol: Type(
                                Type {
                                    attrs: HirSymbolAttrs,
                                    variance: None,
                                    disambiguator: 3,
                                },
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
                                        value: 25,
                                    },
                                ),
                            ),
                        ),
                    ],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 24,
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
                        value: 1,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            MethodFn(
                TraitForTypeMethodFnHirDecl(
                    Id {
                        value: 16,
                    },
                ),
            ),
        ),
    ),
]