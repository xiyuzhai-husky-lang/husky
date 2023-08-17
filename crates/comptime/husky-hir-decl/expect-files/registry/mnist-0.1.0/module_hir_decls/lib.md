[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Enum(
                EnumTypeHirDecl(
                    Id {
                        value: 7,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TupleStruct(
                TupleStructTypeHirDecl(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TupleStruct(
                TupleStructTypeHirDecl(
                    Id {
                        value: 2,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Val(
                ValFugitiveHirDecl {
                    path: FugitivePath(`mnist::input`, `Val`),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist::BinaryImage28`, `Struct`),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                trai: HirTrait {
                    trai_path: TraitPath(`core::visual::Visualize`),
                    template_arguments: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 67,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            MethodFn(
                TraitForTypeMethodFnHirDecl(
                    Id {
                        value: 23,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist`,
                    ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 67,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            AssociatedFn(
                TypeAssociatedFnHirDecl(
                    Id {
                        value: 4,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist`,
                    trai_path: TraitPath(`core::ops::IntIndex`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist::BinaryImage28`, `Struct`),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                trai: HirTrait {
                    trai_path: TraitPath(`core::ops::IntIndex`),
                    template_arguments: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 67,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            AssociatedType(
                TraitForTypeAssociatedTypeHirDecl(
                    Id {
                        value: 3,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                trai: HirTrait {
                    trai_path: TraitPath(`core::visual::Visualize`),
                    template_arguments: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 68,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            MethodFn(
                TraitForTypeMethodFnHirDecl(
                    Id {
                        value: 24,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist`,
                    ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 68,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            AssociatedFn(
                TypeAssociatedFnHirDecl(
                    Id {
                        value: 5,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist`,
                    trai_path: TraitPath(`core::ops::IntIndex`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                trai: HirTrait {
                    trai_path: TraitPath(`core::ops::IntIndex`),
                    template_arguments: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 68,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            AssociatedType(
                TraitForTypeAssociatedTypeHirDecl(
                    Id {
                        value: 4,
                    },
                ),
            ),
        ),
    ),
]