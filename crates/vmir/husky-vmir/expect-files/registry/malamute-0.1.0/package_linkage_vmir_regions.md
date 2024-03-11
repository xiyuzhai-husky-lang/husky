[
    (
        Linkage {
            data: LinkageData::EnumU8ToJsonValue {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
            },
        },
        None,
    ),
    (
        Linkage {
            data: LinkageData::EnumU8ToJsonValue {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
            },
        },
        None,
    ),
    (
        Linkage {
            data: LinkageData::AssocRitchie {
                path: AssocItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 147,
                            },
                        ),
                    ),
                ),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
        Some(
            VmirRegion {
                linkage: Linkage {
                    data: LinkageData::AssocRitchie {
                        path: AssocItemPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 147,
                                    },
                                ),
                            ),
                        ),
                        instantiation: LinInstantiation {
                            symbol_resolutions: [],
                            separator: Some(
                                0,
                            ),
                        },
                    },
                },
                root_expr: 1,
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                            destroyers: ArenaIdxRange(
                                1..1,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Eval,
                    ],
                },
            },
        ),
    ),
    (
        Linkage {
            data: LinkageData::AssocRitchie {
                path: AssocItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 151,
                            },
                        ),
                    ),
                ),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
        Some(
            VmirRegion {
                linkage: Linkage {
                    data: LinkageData::AssocRitchie {
                        path: AssocItemPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 151,
                                    },
                                ),
                            ),
                        ),
                        instantiation: LinInstantiation {
                            symbol_resolutions: [],
                            separator: Some(
                                0,
                            ),
                        },
                    },
                },
                root_expr: 1,
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                            destroyers: ArenaIdxRange(
                                1..1,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Match,
                    ],
                },
            },
        ),
    ),
    (
        Linkage {
            data: LinkageData::UnveilAssocFn {
                path: TraitForTypeItemPath(
                    ItemPathId(
                        Id {
                            value: 151,
                        },
                    ),
                ),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
        Some(
            VmirRegion {
                linkage: Linkage {
                    data: LinkageData::UnveilAssocFn {
                        path: TraitForTypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 151,
                                },
                            ),
                        ),
                        instantiation: LinInstantiation {
                            symbol_resolutions: [],
                            separator: Some(
                                0,
                            ),
                        },
                    },
                },
                root_expr: 1,
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                            destroyers: ArenaIdxRange(
                                1..1,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Match,
                    ],
                },
            },
        ),
    ),
    (
        Linkage {
            data: LinkageData::EnumTypeVariantConstructor {
                path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 101,
                        },
                    ),
                ),
                instantiation: LinInstantiation {
                    symbol_resolutions: [
                        (
                            Type(
                                Type {
                                    attrs: HirTemplateSvarAttrs {
                                        class: Comptime,
                                    },
                                    variance: None,
                                    disambiguator: 0,
                                },
                            ),
                            Explicit(
                                Type(
                                    PathLeading(
                                        LinTypePathLeading(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        (
                            Type(
                                Type {
                                    attrs: HirTemplateSvarAttrs {
                                        class: Comptime,
                                    },
                                    variance: None,
                                    disambiguator: 1,
                                },
                            ),
                            Explicit(
                                Type(
                                    PathLeading(
                                        LinTypePathLeading(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linkage {
            data: LinkageData::EnumTypeVariantConstructor {
                path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 100,
                        },
                    ),
                ),
                instantiation: LinInstantiation {
                    symbol_resolutions: [
                        (
                            Type(
                                Type {
                                    attrs: HirTemplateSvarAttrs {
                                        class: Comptime,
                                    },
                                    variance: None,
                                    disambiguator: 0,
                                },
                            ),
                            Explicit(
                                Type(
                                    PathLeading(
                                        LinTypePathLeading(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        (
                            Type(
                                Type {
                                    attrs: HirTemplateSvarAttrs {
                                        class: Comptime,
                                    },
                                    variance: None,
                                    disambiguator: 1,
                                },
                            ),
                            Explicit(
                                Type(
                                    PathLeading(
                                        LinTypePathLeading(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ],
                    separator: None,
                },
            },
        },
        None,
    ),
]