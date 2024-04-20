```rust
[
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 1,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 2,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 3,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 4,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 5,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 6,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 7,
                },
            ),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::main`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                `<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`,
                TraitItemKind::MethodRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 283,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::raw_contours`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_row_span`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::row_span_sum`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::distribution`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_span_sum`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_right_mass_sum`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
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
    ItemPath::Attr(
        Room32,
        AttrItemPath(
            ItemPathId(
                Id {
                    value: 311,
                },
            ),
        ),
    ),
    ItemPath::TypeVariant(
        Room32,
        TypeVariantPath(
            ItemPathId(
                Id {
                    value: 17,
                },
            ),
        ),
    ),
    ItemPath::TypeVariant(
        Room32,
        TypeVariantPath(
            ItemPathId(
                Id {
                    value: 18,
                },
            ),
        ),
    ),
    ItemPath::TypeVariant(
        Room32,
        TypeVariantPath(
            ItemPathId(
                Id {
                    value: 19,
                },
            ),
        ),
    ),
    ItemPath::TypeVariant(
        Room32,
        TypeVariantPath(
            ItemPathId(
                Id {
                    value: 20,
                },
            ),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlockPath(`mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                `<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`,
                TraitItemKind::MethodRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 285,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::line_segment_sketch`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::bounding_box`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::relative_bounding_box`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::contour_len`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::displacement`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
        ),
    ),
    ItemPath::Attr(
        Room32,
        AttrItemPath(
            ItemPathId(
                Id {
                    value: 318,
                },
            ),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 286,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::from_i_shift28`, `AssocRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::vector`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::to`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::norm`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::dist`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 287,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::point`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::to`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::norm`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::dot`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::cross`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::angle`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::rotation_direction_to`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::angle_to`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 288,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::ClosedRange(0)>::relative_range`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::ClosedRange(0)>::relative_point`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 289,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::relative_bounding_box`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::relative_point`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::xmin`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::xmax`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymin`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymax`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 290,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::xmin`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::xmax`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::ymin`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::ymax`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 36,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 37,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 38,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 39,
                },
            ),
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
        MajorItemPath::Form(
            FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                `<mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)>::visualize`,
                TraitItemKind::MethodRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 292,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::new`, `AssocRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                `<mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)>::visualize`,
                TraitItemKind::MethodRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 294,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::concave_components`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::bounding_box`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::new`, `AssocRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                `<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`,
                TraitItemKind::MethodRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 296,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::norm`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::rel_norm`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::hausdorff_norm`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::angle_change`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::bounding_box`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::relative_bounding_box`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::line_segment`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::start`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::end`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::displacement`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::start_tangent`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::end_tangent`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent as core::visual::Visualize(0)`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                `<mnist_classifier::line_segment_sketch::convex_component::ConvexComponent as core::visual::Visualize(0)>::visualize`,
                TraitItemKind::MethodRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 298,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)>::displacement`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)>::dist_to_point`, `MethodRitchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId(
                    Id {
                        value: 299,
                    },
                ),
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::rel_norm`, `MemoizedField`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::angle_change_norm`, `MemoizedField`),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 54,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 55,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 56,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 57,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 58,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 59,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 60,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 61,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 62,
                },
            ),
        ),
    ),
    ItemPath::Submodule(
        Room32,
        SubmoduleItemPath(
            ItemPathId(
                Id {
                    value: 63,
                },
            ),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::one::hat`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::six::upmost`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::six::bottom1`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::three::back`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::four::left_components`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::four::is_four`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::nine::nine_match`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::nine::downmost`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::nine::big_cc`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::two::two_match`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Ritchie(
                Fn,
            )`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::major::connected_components`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
        ),
    ),
]
```