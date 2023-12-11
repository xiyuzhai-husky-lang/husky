[
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::connected_component`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
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
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::AssociatedFunctionFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::vector`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::norm`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::point`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::to`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle_to`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmin`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmax`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
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
                                    item_kind: MethodFn,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    },
    Linkage {
        data: LinkageData::AssociatedFunctionFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`, `AssociatedFunctionFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
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
                                    item_kind: MethodFn,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    },
    Linkage {
        data: LinkageData::AssociatedFunctionFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`, `AssociatedFunctionFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
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
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::three::uparc`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::three::downarc`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::three::back`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::nine::downmost`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::nine::big_cc`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::FunctionFnItem(
            FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `core::clone`,
                                            trai_path: TraitPath(`core::clone::Clone`),
                                            ty_sketch: TypeSketch::DeriveAny,
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `clone`,
                                    item_kind: MethodFn,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `core::clone`,
                                            trai_path: TraitPath(`core::clone::Clone`),
                                            ty_sketch: TypeSketch::DeriveAny,
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `clone`,
                                    item_kind: MethodFn,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::first`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `core::visual`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::DeriveAny,
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
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `core::visual`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::DeriveAny,
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
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
            ),
        ),
    },
]