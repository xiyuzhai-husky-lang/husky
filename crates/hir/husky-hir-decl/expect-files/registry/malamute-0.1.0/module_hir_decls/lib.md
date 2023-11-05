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
                            HirTemplateParameter {
                                data: Constant {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 547,
                                            },
                                        ),
                                    ),
                                    ty: Symbol(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
                            },
                        ],
                    ),
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
                    ty_sketch: TypeSketch::Path(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters(
                    [
                        HirTemplateParameter {
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
                        HirTemplateParameter {
                            data: Constant {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 547,
                                        },
                                    ),
                                ),
                                ty: Symbol(
                                    Type {
                                        attrs: HirSymbolAttrs,
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                            },
                        },
                    ],
                ),
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
                path: TraitForTypeImplBlockPath {
                    module_path: `malamute`,
                    trai_path: TraitPath(`core::ops::Unveil`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters(
                    [
                        HirTemplateParameter {
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
                        HirTemplateParameter {
                            data: Constant {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 547,
                                        },
                                    ),
                                ),
                                ty: Symbol(
                                    Type {
                                        attrs: HirSymbolAttrs,
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                            },
                        },
                    ],
                ),
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