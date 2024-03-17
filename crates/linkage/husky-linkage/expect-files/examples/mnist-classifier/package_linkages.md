```rust
[
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::main`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 25,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 27,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 28,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 29,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 31,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 39,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::raw_contours`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_row_span`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::row_span_sum`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::distribution`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_span_sum`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_right_mass_sum`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 88,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 54,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::EnumU8ToJsonValue {
            ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 17,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 17,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 18,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 18,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 19,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 19,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 20,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 20,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 143,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 144,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::line_segment_sketch`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::bounding_box`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::relative_bounding_box`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::contour_len`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::displacement`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 74,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 75,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 74,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 75,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 74,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 75,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 101,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 57,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 194,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 195,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 194,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 195,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::AssocRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::from_i_shift28`, `AssocRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::vector`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::to`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::norm`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Point2d(0)>::dist`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::point`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::to`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::norm`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::dot`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::cross`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::angle`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::rotation_direction_to`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::angle_to`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::ClosedRange(0)>::relative_range`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::ClosedRange(0)>::relative_point`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::relative_bounding_box`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::relative_point`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::xmin`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::xmax`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymin`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymax`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::xmin`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::xmax`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::ymin`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::ymax`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 54,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 108,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 109,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 154,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 206,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::AssocRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::new`, `AssocRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::concave_components`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::bounding_box`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::AssocRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::new`, `AssocRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 5,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 206,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::norm`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::rel_norm`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::hausdorff_norm`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::angle_change`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::bounding_box`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::relative_bounding_box`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::line_segment`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::start`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::end`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::displacement`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::start_tangent`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::end_tangent`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 5,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 232,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 108,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 109,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)>::displacement`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)>::dist_to_point`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructConstructor {
            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 31,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                template_arguments: [],
            },
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 267,
                        },
                    ),
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::rel_norm`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MemoizedField {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::angle_change_norm`, `MemoizedField`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::one::hat`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::three::downarc`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::three::back`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieEager {
            path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::MajorVal {
            path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::AssocRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                    TraitItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    2,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::UnveilAssocFn {
            path: TraitForTypeItemPath(
                `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                TraitItemKind::AssocRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    2,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<#derive _ as core::clone::Clone(0)>::clone`,
                    TraitItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            SelfType,
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 20,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::push`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::collect_leashes`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`core::option::Option`, `Enum`),
                    template_arguments: [
                        LinTemplateArgument::Type(
                            LinType::PathLeading(
                                LinTypePathLeading {
                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                    template_arguments: [
                                        LinTemplateArgument::Type(
                                            LinType::PathLeading(
                                                LinTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    template_arguments: [],
                                                },
                                            ),
                                        ),
                                    ],
                                },
                            ),
                        ),
                    ],
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::pop_with_largest_opt_f32`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 21,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::push`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 22,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::last`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfPlace,
                        ),
                        SelfQual(
                            Ref,
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::last`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfPlace,
                        ),
                        SelfQual(
                            RefMut,
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::push`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::pop`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::push`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::first`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<#derive _ as core::clone::Clone(0)>::clone`,
                    TraitItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            SelfType,
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::last`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::last`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfPlace,
                        ),
                        SelfQual(
                            Ref,
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::last`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfPlace,
                        ),
                        SelfQual(
                            RefMut,
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::pop`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::end`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::push`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::first`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfPlace,
                        ),
                        SelfQual(
                            Ref,
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::first`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfPlace,
                        ),
                        SelfQual(
                            RefMut,
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::cyclic_slice_leashed`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::cyclic_slice_leashed`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::push`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::first`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::end`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::last`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::collect_leashes`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`core::option::Option`, `Enum`),
                    template_arguments: [
                        LinTemplateArgument::Type(
                            LinType::PathLeading(
                                LinTypePathLeading {
                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                    template_arguments: [
                                        LinTemplateArgument::Type(
                                            LinType::PathLeading(
                                                LinTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    template_arguments: [],
                                                },
                                            ),
                                        ),
                                    ],
                                },
                            ),
                        ),
                    ],
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                Ritchie(
                                    LinkageRitchieType(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::pop_with_largest_opt_f32`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 23,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::push`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 24,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            SelfLifetime,
                        ),
                        SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 23,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::VecConstructor {
            element_ty: LinType::Ritchie(
                LinkageRitchieType {
                    parameters: [
                        LinkageRitchieParameter {
                            contract: Pure,
                            parameter_ty: PathLeading(
                                LinTypePathLeading(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        },
                    ],
                    return_ty: LinType::PathLeading(
                        LinTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                LinTemplateArgument::Type(
                                    LinType::PathLeading(
                                        LinTypePathLeading {
                                            ty_path: TypePath(`core::num::f32`, `Extern`),
                                            template_arguments: [],
                                        },
                                    ),
                                ),
                            ],
                        },
                    ),
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::MajorRitchieLazy {
            path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                Gn,
            )`),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::TypeDefault {
            ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linkage {
        data: LinkageData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                    Fn,
                )`),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::option::Option`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 206,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::option::Option`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 206,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::option::Option`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 206,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantField {
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 206,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field: Tuple {
                index: 0,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::option::Option`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 207,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::option::Option`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 207,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::Class`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 249,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::Class`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 249,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::Class`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 249,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantField {
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 249,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field: Tuple {
                index: 0,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::Class`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 250,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::Class`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 250,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                template_arguments: [
                                    LinTemplateArgument::Type(
                                        LinType::PathLeading(
                                            LinTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 204,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                template_arguments: [
                                    LinTemplateArgument::Type(
                                        LinType::PathLeading(
                                            LinTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 204,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                template_arguments: [
                                    LinTemplateArgument::Type(
                                        LinType::PathLeading(
                                            LinTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 204,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantField {
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 204,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field: Tuple {
                index: 0,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                template_arguments: [
                                    LinTemplateArgument::Type(
                                        LinType::PathLeading(
                                            LinTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 205,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                template_arguments: [
                                    LinTemplateArgument::Type(
                                        LinType::PathLeading(
                                            LinTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 205,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                template_arguments: [
                                    LinTemplateArgument::Type(
                                        LinType::PathLeading(
                                            LinTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                            },
                                        ),
                                    ),
                                ],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 205,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantField {
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 205,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field: Tuple {
                index: 0,
            },
        },
    },
]
```