[
    Some(
        ValkyrieRides {
            hir_template_parameters: None,
            rides: VecSet {
                data: [],
            },
        },
    ),
    Some(
        ValkyrieRides {
            hir_template_parameters: Some(
                HirTemplateParameters(
                    [],
                ),
            ),
            rides: VecSet {
                data: [],
            },
        },
    ),
    Some(
        ValkyrieRides {
            hir_template_parameters: None,
            rides: VecSet {
                data: [
                    ValkyrieRide {
                        linkage_path: TypeItem(
                            TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 222,
                                    },
                                ),
                            ),
                        ),
                        instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    Explicit(
                                        Type(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 34,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        linkage_path: TypeItem(
                            TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 222,
                                    },
                                ),
                            ),
                        ),
                        instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    Explicit(
                                        Type(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        },
    ),
]