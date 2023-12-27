[
    Linkage {
        data: LinkageData::FunctionFnItem {
            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::FunctionFnItem {
            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinkageType::PathLeading(
                LinkageTypePathLeading {
                    ty_path: TypePath(`core::num::i32`, `Extern`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinkageType::PathLeading(
                LinkageTypePathLeading {
                    ty_path: TypePath(`core::mem::Ref`, `Extern`),
                    template_arguments: [
                        LinkageTemplateArgument::Constant(
                            LinkageConstant(
                                StaticLifetime,
                            ),
                        ),
                        LinkageTemplateArgument::Type(
                            LinkageType::PathLeading(
                                LinkageTypePathLeading {
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