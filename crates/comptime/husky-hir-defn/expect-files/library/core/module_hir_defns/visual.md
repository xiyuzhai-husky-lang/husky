[
    HirDefn::MajorItem(
        MajorItemHirDefn::Trait(
            TraitHirDefn {
                path: TraitPath(`core::visual::Visualize`),
                hir_decl: TraitHirDecl {
                    path: TraitPath(`core::visual::Visualize`),
                    template_parameters: HirTemplateParameters {
                        data: [],
                    },
                },
            },
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternTypeHirDefn {
                    path: TypePath(`core::visual::Html`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::visual::Html`, `Extern`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        hir_expr_region: HirEagerExprRegion,
                    },
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath {
                        impl_block: TraitForTypeImplBlockPath {
                            module_path: `core::visual`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::DeriveAny,
                            disambiguator: 0,
                        },
                        ident: `visualize`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `core::visual`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::DeriveAny,
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_value_parameter: HirRitchieParameter {
                            contract: None,
                            ty: Symbol(
                                SelfType,
                            ),
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [],
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 27,
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