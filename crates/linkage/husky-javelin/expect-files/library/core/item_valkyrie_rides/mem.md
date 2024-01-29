[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::mem::Ref`, `Extern`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
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
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::mem::RefMut`, `Extern`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
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
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::mem::Leash`, `Extern`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
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
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::mem::At`, `Extern`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
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
                ),
                rides: [],
            },
        ),
    ),
    (
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
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
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
                ),
                rides: [],
            },
        ),
    ),
]