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
                                symbol: Lifetime(
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
                                data: Lifetime {
                                    label: Label {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
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
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
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
                                symbol: Lifetime(
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
                                data: Lifetime {
                                    label: Label {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
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
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
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
                                symbol: Type(
                                    Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
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
                                symbol: Place(
                                    HirPlaceSymbol {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: Place {
                                    label: Label {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 49,
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
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
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
                                symbol: Type(
                                    Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
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
                ),
                rides: [],
            },
        ),
    ),
]