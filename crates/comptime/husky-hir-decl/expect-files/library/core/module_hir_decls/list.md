[
    HirDecl::ModuleItem(
        ModuleItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 7,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
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
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 2,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 3,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 4,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 5,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 6,
                    },
                ),
            ),
        ),
    ),
]