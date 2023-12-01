[
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlock {
                data: TraitForTypeImplBlockPathData {
                    module_path: `mnist_classifier::raw_contour`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                    disambiguator: 0,
                },
            },
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssociatedItem(
                        AssociatedItemPathData::TraitForTypeItem(
                            TraitForTypeItemPathData {
                                impl_block: TraitForTypeImplBlock {
                                    data: TraitForTypeImplBlockPathData {
                                        module_path: `mnist_classifier::raw_contour`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                ident: `visualize`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId {
                    data: ItemPathData::ImplBlock(
                        ImplBlockPathData::TypeImplBlock(
                            TypeImplBlockPathData {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssociatedItem(
                        AssociatedItemPathData::TypeItem(
                            TypeItemPathData {
                                impl_block: TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                                ident: `line_segment_sketch`,
                                item_kind: MemoizedField,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssociatedItem(
                        AssociatedItemPathData::TypeItem(
                            TypeItemPathData {
                                impl_block: TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                                ident: `bounding_box`,
                                item_kind: MemoizedField,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssociatedItem(
                        AssociatedItemPathData::TypeItem(
                            TypeItemPathData {
                                impl_block: TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                                ident: `relative_bounding_box`,
                                item_kind: MemoizedField,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssociatedItem(
                        AssociatedItemPathData::TypeItem(
                            TypeItemPathData {
                                impl_block: TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                                ident: `contour_len`,
                                item_kind: MemoizedField,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssociatedItem(
                        AssociatedItemPathData::TypeItem(
                            TypeItemPathData {
                                impl_block: TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                                ident: `displacement`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]