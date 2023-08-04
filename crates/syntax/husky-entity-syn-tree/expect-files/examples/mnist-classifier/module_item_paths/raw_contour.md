Ok(
    [
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
                    module_path: `mnist_classifier::raw_contour`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: Path(
                        TypePath(
                            Id {
                                value: 82,
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
                        module_path: `mnist_classifier::raw_contour`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: Path(
                            TypePath(
                                Id {
                                    value: 82,
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
                    module_path: `mnist_classifier::raw_contour`,
                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `line_segment_sketch`,
                    item_kind: MemoizedField,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `relative_bounding_box`,
                    item_kind: MemoizedField,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `contour_len`,
                    item_kind: MemoizedField,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `displacement`,
                    item_kind: MethodFn,
                },
            ),
        ),
    ],
)