[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::raw_bits::r32`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::raw_bits::r32`, `Extern`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        hir_expr_region: HirEagerExprRegion,
                    },
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `core::raw_bits`,
                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 16,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::raw_bits`,
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `last_bits`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `last_bits`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 16,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_value_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [
                                HirRitchieParameter {
                                    contract: None,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 16,
                                },
                            ),
                        ),
                    },
                    body: None,
                    hir_expr_region: HirEagerExprRegion,
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::raw_bits`,
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `ctz`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `ctz`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 16,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_value_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [],
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 5,
                                },
                            ),
                        ),
                    },
                    body: None,
                    hir_expr_region: HirEagerExprRegion,
                },
            ),
        ),
    ),
]