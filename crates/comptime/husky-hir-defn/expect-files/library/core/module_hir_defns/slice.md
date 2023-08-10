[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::slice::Slice`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::slice::Slice`, `Extern`),
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                    module_path: `core::slice`,
                    ty_path: TypePath(`core::slice::Slice`, `Extern`),
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
                            value: 24,
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
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `len`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::slice`,
                                ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `len`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 24,
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
                                        value: 24,
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
                                    value: 1,
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
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `swap`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::slice`,
                                ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `swap`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 24,
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
                                        value: 24,
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
                                                value: 1,
                                            },
                                        ),
                                    ),
                                },
                                HirRitchieParameter {
                                    contract: None,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 1,
                                            },
                                        ),
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
    HirDefn::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `core::slice`,
                    ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                            value: 8,
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
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `ilen`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::slice`,
                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `ilen`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 8,
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
                                        value: 8,
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
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `start`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::slice`,
                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `start`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 8,
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
                                        value: 8,
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
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `end`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::slice`,
                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `end`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 8,
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
                                        value: 8,
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
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `first`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::slice`,
                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `first`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 8,
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
                                        value: 8,
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
                                    value: 25,
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
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `last`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::slice`,
                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `last`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 8,
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
                                        value: 8,
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
                                    value: 25,
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