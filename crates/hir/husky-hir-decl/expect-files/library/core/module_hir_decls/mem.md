[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 10,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 11,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 12,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 13,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
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
        ),
    ),
]