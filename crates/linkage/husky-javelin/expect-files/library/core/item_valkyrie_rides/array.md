[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::array::Array`, `Extern`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateSymbol::Const(
                                    HirConstSymbol {
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::usize`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                        index: HirConstSymbolIndex::PathLeading {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            disambiguator: 0,
                                            ty_path: TypePath(`core::num::usize`, `Extern`),
                                        },
                                    },
                                ),
                                data: HirTemplateParameterData::Constant {
                                    ident: `L`,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::usize`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
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
                ),
                rides: [],
            },
        ),
    ),
]