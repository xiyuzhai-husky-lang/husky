[
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::connected_component`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::geom2d`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::fermi`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::major`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::main`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TraitForTypeImplBlock(
                    TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist_classifier::connected_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 408,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TypeImplBlock(
                    TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 409,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 410,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 411,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 412,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 413,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 414,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 415,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 416,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 417,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 418,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 419,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 420,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 421,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 422,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 423,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 424,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TypeImplBlock(
                    TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 425,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 426,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 427,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 428,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 429,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TypeImplBlock(
                    TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 430,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 431,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 432,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 433,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 434,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 435,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 436,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 437,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TypeImplBlock(
                    TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 438,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 439,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TypeImplBlock(
                    TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 440,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 441,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 442,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 443,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 444,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 445,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TypeImplBlock(
                    TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 446,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 447,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 448,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 449,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 450,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 451,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 452,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 453,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
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
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 454,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 455,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 456,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TraitForTypeImplBlock(
                    TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 457,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TypeImplBlock(
                    TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 458,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 459,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 460,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 461,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 462,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 463,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 464,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 465,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 466,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 467,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 468,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 469,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TraitForTypeImplBlock(
                    TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 470,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TypeImplBlock(
                    TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 471,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 472,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::ImplBlock(
                ImplBlockPath::TypeImplBlock(
                    TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::fermi`,
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 473,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 474,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::AssociatedItem(
                AssociatedItemPath::TypeItem(
                    TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 475,
                            },
                        ),
                    ),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::five`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits::two`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::three::uparc`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::three::downarc`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::three::back`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::nine::downmost`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::nine::big_cc`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::Item {
            item_path: ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                ),
            ),
            instantiation: LinkageInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
]