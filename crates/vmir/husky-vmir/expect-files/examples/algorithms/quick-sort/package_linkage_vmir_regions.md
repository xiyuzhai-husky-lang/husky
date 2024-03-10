[
    (
        Linkage {
            data: LinkageData::MajorRitchieEager {
                path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                    Fn,
                )`),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        Some(
            VmirRegion {
                linkage: Linkage {
                    data: LinkageData::MajorRitchieEager {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                            Fn,
                        )`),
                        instantiation: LinInstantiation {
                            symbol_resolutions: [],
                            separator: None,
                        },
                    },
                },
                root_expr: 1,
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Block,
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [],
                },
            },
        ),
    ),
    (
        Linkage {
            data: LinkageData::MajorRitchieEager {
                path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                    Fn,
                )`),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        Some(
            VmirRegion {
                linkage: Linkage {
                    data: LinkageData::MajorRitchieEager {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                            Fn,
                        )`),
                        instantiation: LinInstantiation {
                            symbol_resolutions: [],
                            separator: None,
                        },
                    },
                },
                root_expr: 1,
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Block,
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [],
                },
            },
        ),
    ),
    (
        Linkage {
            data: LinkageData::VecConstructor {
                element_ty: LinType::PathLeading(
                    LinTypePathLeading {
                        ty_path: TypePath(`core::num::i32`, `Extern`),
                        template_arguments: [],
                    },
                ),
            },
        },
        None,
    ),
    (
        Linkage {
            data: LinkageData::VecConstructor {
                element_ty: LinType::PathLeading(
                    LinTypePathLeading {
                        ty_path: TypePath(`core::mem::Ref`, `Extern`),
                        template_arguments: [
                            LinTemplateArgument::Constant(
                                LinConstant(
                                    StaticLifetime,
                                ),
                            ),
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::str::str`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ],
                    },
                ),
            },
        },
        None,
    ),
]