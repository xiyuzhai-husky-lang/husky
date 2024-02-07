[
    Linkage {
        data: LinkageData::MajorFn {
            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorFn {
            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
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
]