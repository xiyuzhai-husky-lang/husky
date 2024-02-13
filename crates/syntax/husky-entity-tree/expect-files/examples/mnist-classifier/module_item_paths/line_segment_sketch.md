[
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId {
                data: ItemPathData::SubmoduleItem(
                    SubmoduleItemPathData {
                        submodule_path: SubmodulePath(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                ),
            },
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId {
                data: ItemPathData::SubmoduleItem(
                    SubmoduleItemPathData {
                        submodule_path: SubmodulePath(
                            `mnist_classifier::line_segment_sketch::convex_component`,
                        ),
                    },
                ),
            },
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId {
                data: ItemPathData::SubmoduleItem(
                    SubmoduleItemPathData {
                        submodule_path: SubmodulePath(
                            `mnist_classifier::line_segment_sketch::convexity`,
                        ),
                    },
                ),
            },
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId {
                data: ItemPathData::SubmoduleItem(
                    SubmoduleItemPathData {
                        submodule_path: SubmodulePath(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                    },
                ),
            },
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlock {
                data: TraitForTypeImplBlockPathData {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    ),
                    disambiguator: 0,
                },
            },
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssocItem(
                        AssocItemPathData::TraitForTypeItem(
                            TraitForTypeItemPathData {
                                impl_block: TraitForTypeImplBlock {
                                    data: TraitForTypeImplBlockPathData {
                                        module_path: `mnist_classifier::line_segment_sketch`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                ident: `visualize`,
                                item_kind: MethodRitchie(
                                    Fn,
                                ),
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
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`, `AssocRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlock {
                data: TraitForTypeImplBlockPathData {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    ),
                    disambiguator: 0,
                },
            },
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssocItem(
                        AssocItemPathData::TraitForTypeItem(
                            TraitForTypeItemPathData {
                                impl_block: TraitForTypeImplBlock {
                                    data: TraitForTypeImplBlockPathData {
                                        module_path: `mnist_classifier::line_segment_sketch`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                ident: `visualize`,
                                item_kind: MethodRitchie(
                                    Fn,
                                ),
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
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`, `AssocRitchie(
                Fn,
            )`),
        ),
    ),
]