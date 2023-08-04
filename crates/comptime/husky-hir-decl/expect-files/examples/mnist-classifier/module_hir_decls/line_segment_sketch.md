[
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: `mnist_classifier::line_segment_sketch::concave_component`,
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: `mnist_classifier::line_segment_sketch::convex_component`,
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: `mnist_classifier::line_segment_sketch::convexity`,
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: `mnist_classifier::line_segment_sketch::line_segment`,
        },
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            PropsStruct(
                PropsStructTypeHirDecl(
                    Id {
                        value: 14,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            PropsStruct(
                PropsStructTypeHirDecl(
                    Id {
                        value: 15,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Fn(
                FnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                    template_parameters: HirTemplateParameters {
                        data: [],
                    },
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Fn(
                FnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                    template_parameters: HirTemplateParameters {
                        data: [],
                    },
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Fn(
                FnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                    template_parameters: HirTemplateParameters {
                        data: [],
                    },
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Fn(
                FnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                    template_parameters: HirTemplateParameters {
                        data: [],
                    },
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Fn(
                FnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                    template_parameters: HirTemplateParameters {
                        data: [],
                    },
                },
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: Path(
                        TypePath(
                            Id {
                                value: 91,
                            },
                        ),
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
                            value: 54,
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
                        value: 20,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 54,
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
                        value: 60,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: Path(
                        TypePath(
                            Id {
                                value: 92,
                            },
                        ),
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
                            value: 52,
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
                        value: 21,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 52,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MemoizedField(
                TypeMemoizedFieldHirDecl(
                    Id {
                        value: 18,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MemoizedField(
                TypeMemoizedFieldHirDecl(
                    Id {
                        value: 19,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            AssociatedFn(
                TypeAssociatedFnHirDecl(
                    Id {
                        value: 3,
                    },
                ),
            ),
        ),
    ),
]