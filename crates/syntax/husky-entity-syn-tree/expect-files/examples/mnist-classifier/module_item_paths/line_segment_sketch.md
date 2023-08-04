Ok(
    [
        ItemPath::Submodule(
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::concave_component`,
            ),
        ),
        ItemPath::Submodule(
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::convex_component`,
            ),
        ),
        ItemPath::Submodule(
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::convexity`,
            ),
        ),
        ItemPath::Submodule(
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::line_segment`,
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
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
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
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
                    ident: `visualize`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `new`,
                    item_kind: AssociatedFn,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `displacement`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
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
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
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
                    ident: `visualize`,
                    item_kind: MethodFn,
                },
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `concave_components`,
                    item_kind: MemoizedField,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `bounding_box`,
                    item_kind: MemoizedField,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `new`,
                    item_kind: AssociatedFn,
                },
            ),
        ),
    ],
)