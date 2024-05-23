```rust
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
    ItemPath::Attr(
        Room32,
        AttrItemPath(`mnist_classifier::raw_contour::Direction::@derive(0)`),
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
            TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(
                `mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`,
                TypeItemKind::MemoizedField,
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(
                `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                TypeItemKind::MemoizedField,
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(
                `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                TypeItemKind::MemoizedField,
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(
                `mnist_classifier::raw_contour::RawContour(0)::contour_len`,
                TypeItemKind::MemoizedField,
            ),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(
                `mnist_classifier::raw_contour::RawContour(0)::displacement`,
                TypeItemKind::MethodRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
        ),
    ),
]
```