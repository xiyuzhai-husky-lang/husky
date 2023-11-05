[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::raw_bits::r32`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::raw_bits::r32`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_expr_region: HirEagerExprRegion {
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath {
                        module_path: `core::raw_bits`,
                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                        disambiguator: 0,
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: PathLeading(
                        HirTypePathLeading(
                            Id {
                                value: 16,
                            },
                        ),
                    ),
                },
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
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: Ordinary,
                        parenate_parameters: HirParenateParameters(
                            [
                                Ordinary,
                            ],
                        ),
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 16,
                                },
                            ),
                        ),
                    },
                    eager_body_with_hir_eager_expr_region: None,
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
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: Ordinary,
                        parenate_parameters: HirParenateParameters(
                            [],
                        ),
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 5,
                                },
                            ),
                        ),
                    },
                    eager_body_with_hir_eager_expr_region: None,
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
                        ident: `co`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `co`,
                            item_kind: MethodFn,
                        },
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: Ordinary,
                        parenate_parameters: HirParenateParameters(
                            [],
                        ),
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 5,
                                },
                            ),
                        ),
                    },
                    eager_body_with_hir_eager_expr_region: None,
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
                        ident: `span`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `span`,
                            item_kind: MethodFn,
                        },
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: Ordinary,
                        parenate_parameters: HirParenateParameters(
                            [],
                        ),
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 5,
                                },
                            ),
                        ),
                    },
                    eager_body_with_hir_eager_expr_region: None,
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
                        ident: `right_mass`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `right_mass`,
                            item_kind: MethodFn,
                        },
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: Ordinary,
                        parenate_parameters: HirParenateParameters(
                            [],
                        ),
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 5,
                                },
                            ),
                        ),
                    },
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
]