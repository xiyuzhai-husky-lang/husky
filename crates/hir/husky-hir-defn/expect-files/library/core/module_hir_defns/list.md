[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::list::List`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::list::List`, `Extern`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: Some(
                                                Covariant,
                                            ),
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
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
                    module_path: `core::list`,
                    ty_path: TypePath(`core::list::List`, `Extern`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [
                        HirTemplateParameter {
                            symbol: Type(
                                Type {
                                    attrs: HirSymbolAttrs,
                                    variance: None,
                                    disambiguator: 0,
                                },
                            ),
                            traits: [],
                        },
                    ],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 2,
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
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `ilen`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `ilen`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 2,
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
                                    value: 3,
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
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `push`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `push`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_parameter: HirRitchieParameter {
                            contract: BorrowMut,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [
                                HirRitchieParameter {
                                    contract: None,
                                    ty: Symbol(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
                            ],
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 4,
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
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `first`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `first`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 2,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `last`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `last`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 2,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `pop`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `pop`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_parameter: HirRitchieParameter {
                            contract: BorrowMut,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 2,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `collect_leashes`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `collect_leashes`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_parameter: HirRitchieParameter {
                            contract: Leash,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 2,
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
                                    value: 7,
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
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `cyclic_slice_leashed`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `cyclic_slice_leashed`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_parameter: HirRitchieParameter {
                            contract: Leash,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 2,
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
                                                value: 3,
                                            },
                                        ),
                                    ),
                                },
                                HirRitchieParameter {
                                    contract: None,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 8,
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