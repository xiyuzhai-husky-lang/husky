```rust
[
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::main`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::main`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::EffHoles`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::hole_tmpl`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::find_connected_components`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::distribution`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::distribution`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::RawContour`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::EnumUnitToJsonValue {
            ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Up`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::Direction::Up`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Up`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::Direction::Up`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Left`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::Direction::Left`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Left`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::Direction::Left`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Down`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::Direction::Down`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Down`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::Direction::Down`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Right`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::Direction::Right`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Right`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::Direction::Right`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::get_pixel_pair`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::get_inward_direction`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::get_angle_change`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::get_outward_direction`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::StreakCache`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::get_concave_middle_point`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::find_raw_contours`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::RawContour(0)::bounding_box`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::raw_contour::RawContour(0)::contour_len`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::RawContour(0)::contour_len`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::raw_contour::RawContour(0)::displacement`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Point2d`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::RelativePoint2d`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Vector2d`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::ClosedRange`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::BoundingBox`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::AssocRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Point2d(0)::from_i_shift28`,
                    TypeItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Point2d(0)::from_i_shift28`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Point2d(0)::vector`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Point2d(0)::vector`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Point2d(0)::to`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Point2d(0)::norm`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Point2d(0)::norm`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Point2d(0)::dist`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Point2d(0)::dist`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Vector2d(0)::point`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Vector2d(0)::point`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Vector2d(0)::to`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Vector2d(0)::to`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Vector2d(0)::norm`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Vector2d(0)::dot`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Vector2d(0)::dot`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Vector2d(0)::cross`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Vector2d(0)::angle`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::Vector2d(0)::angle_to`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle_to`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::ClosedRange(0)::relative_range`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_range`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::ClosedRange(0)::relative_point`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_point`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::BoundingBox(0)::relative_point`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_point`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::BoundingBox(0)::xmin`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::xmin`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::BoundingBox(0)::xmax`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::xmax`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::BoundingBox(0)::ymin`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::ymin`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::BoundingBox(0)::ymax`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::ymax`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::go_right`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::go_left`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::extend_end`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::extend_start`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::find_line_segments`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::AssocRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`,
                    TypeItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::AssocRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`,
                    TypeItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Copyable,
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
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::StructConstructor {
            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::StructDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                template_arguments: [],
            },
        },
    },
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::StructField {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                template_arguments: [],
            },
            field_ty_leash_class: Other,
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
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::fermi::fermi_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)::norm`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::Memo {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::zero::open_one_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::zero::almost_closed`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::zero::is_zero`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::one::one_fermi_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::one::is_one`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::one::is_one`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::one::upmost`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::one::downmost`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::one::hat`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::one::hat`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::six::six_match`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::six::six_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::six::six_match_refined1`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::six::is_six`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::six::is_six`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::six::upmost`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::six::upmost`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::six::bottom1`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::six::bottom1`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::three::three_fermi_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::three::is_three`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::three::is_three`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::three::uparc`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::three::downarc`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::three::back`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::three::back`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::four::left_components`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::four::left_components`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::four::components_max_downwards`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::four::components_max_heights`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::four::is_four`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::four::is_four`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::four::displacement_downwards`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::four::cc_box_heights`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::five::is_five`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::five::is_five`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::seven::simple_seven_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::seven::special_seven_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::seven::leftupcc_pattern`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::seven::leftdowncc_pattern`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::seven::is_seven`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::eight::upper_mouth_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::eight::is_eight`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::nine::nine_match`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::nine::nine_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::nine::nine_match_refine`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::nine::is_nine`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::nine::downmost`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::nine::downmost`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::nine::big_cc`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::nine::big_cc`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::two::two_match`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::two::two_match`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::two::left_cc_pattern`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::two::right_cc_pattern`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::two::down_cc_pattern`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::digits::two::is_two`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::digits::two::is_two`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::major::connected_components`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::major::connected_components`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::major::major_connected_component`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::major::major_raw_contours`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::major::major_raw_contour`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::major::major_line_segment_sketch`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorVal {
            path: MajorFormPath(`mnist_classifier::major::major_concave_components`, `Val`),
            instantiation: LinInstantiation {
                path: ItemPath(`mnist_classifier::major::major_concave_components`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::AssocRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                    TraitItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::UnveilAssocRitchie {
            path: TraitForTypeItemPath(
                `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                TraitItemKind::AssocRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<#derive _ as core::clone::Clone(0)>::clone`,
                    TraitItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::SelfType,
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::push`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::push`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::collect_leashes`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::collect_leashes`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::VecConstructor {
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::pop_with_largest_opt_f32`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::push`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::push`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
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
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::ilen`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::ilen`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::ilen`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::ilen`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linket {
        data: LinketData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::last`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::last`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Quary(
                            SelfPlace,
                        ),
                        LinTermVariableResolution::SelfQual(
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::last`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::last`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Quary(
                            SelfPlace,
                        ),
                        LinTermVariableResolution::SelfQual(
                            Mut,
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::push`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::push`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::pop`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::pop`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::push`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::push`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::slice::CyclicSlice(0)::first`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::slice::CyclicSlice(0)::first`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<#derive _ as core::clone::Clone(0)>::clone`,
                    TraitItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::SelfType,
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::slice::CyclicSlice(0)::last`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::slice::CyclicSlice(0)::last`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::ilen`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::ilen`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::last`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::last`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Quary(
                            SelfPlace,
                        ),
                        LinTermVariableResolution::SelfQual(
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::last`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::last`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Quary(
                            SelfPlace,
                        ),
                        LinTermVariableResolution::SelfQual(
                            Mut,
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::slice::CyclicSlice(0)::start`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::slice::CyclicSlice(0)::start`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::pop`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::pop`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::slice::CyclicSlice(0)::end`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::slice::CyclicSlice(0)::end`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::push`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::push`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::first`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::first`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Quary(
                            SelfPlace,
                        ),
                        LinTermVariableResolution::SelfQual(
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::first`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::first`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Quary(
                            SelfPlace,
                        ),
                        LinTermVariableResolution::SelfQual(
                            Mut,
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::cyclic_slice_leashed`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::VecConstructor {
            element_ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::cyclic_slice_leashed`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::push`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::push`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::slice::CyclicSlice(0)::first`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::slice::CyclicSlice(0)::first`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::slice::CyclicSlice(0)::start`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::slice::CyclicSlice(0)::start`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::slice::CyclicSlice(0)::end`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::slice::CyclicSlice(0)::end`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::slice::CyclicSlice(0)::last`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::slice::CyclicSlice(0)::last`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::collect_leashes`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::collect_leashes`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::VecConstructor {
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::ilen`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::ilen`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::Ritchie(
                                    LinRitchieType {
                                        parameters: [
                                            LinRitchieParameter {
                                                contract: Pure,
                                                parameter_ty: PathLeading(
                                                    LinTypePathLeading(
                                                        Id {
                                                            value: 32,
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::pop_with_largest_opt_f32`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::push`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::push`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
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
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Lifetime(
                            SelfLifetime,
                        ),
                        LinTermVariableResolution::SelfLifetime,
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::ilen`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::ilen`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                ],
                separator: Some(
                    1,
                ),
            },
        },
    },
    Linket {
        data: LinketData::VecConstructor {
            element_ty: LinType::Ritchie(
                LinRitchieType {
                    parameters: [
                        LinRitchieParameter {
                            contract: Pure,
                            parameter_ty: PathLeading(
                                LinTypePathLeading(
                                    Id {
                                        value: 32,
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
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::ilen`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::ilen`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`malamute::narrow_down`, `Ritchie(
                Gn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::narrow_down`),
                context: LinTypeContext {
                    comptime_var_overrides: [
                        (
                            MajorItem(
                                Form(
                                    MajorFormPath(
                                        ItemPathId(
                                            Id {
                                                value: 116,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::TypeDefault {
            ty: LinType::PathLeading(
                LinTypePathLeading {
                    ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                    template_arguments: [],
                },
            ),
        },
    },
    Linket {
        data: LinketData::MethodRitchie {
            path: AssocItemPath::TypeItem(
                TypeItemPath(
                    `core::vec::Vec(0)::ilen`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`core::vec::Vec(0)::ilen`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        template_arguments: [],
                                    },
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
    Linket {
        data: LinketData::EnumVariantConstructor {
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
            path: TypeVariantPath(`core::option::Option::Some`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::option::Option::Some`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
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
            path: TypeVariantPath(`core::option::Option::Some`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::option::Option::Some`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
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
            path: TypeVariantPath(`core::option::Option::Some`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::option::Option::Some`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`core::option::Option::Some`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::option::Option::Some`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field_ty_leash_class: Copyable,
            field: Tuple {
                index: 0,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
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
            path: TypeVariantPath(`core::option::Option::None`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::option::Option::None`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
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
            path: TypeVariantPath(`core::option::Option::None`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::option::Option::None`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
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
            path: TypeVariantPath(`malamute::Class::Known`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::Class::Known`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
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
            path: TypeVariantPath(`malamute::Class::Known`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::Class::Known`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
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
            path: TypeVariantPath(`malamute::Class::Known`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::Class::Known`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`malamute::Class::Known`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::Class::Known`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field_ty_leash_class: Other,
            field: Tuple {
                index: 0,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
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
            path: TypeVariantPath(`malamute::Class::Unknown`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::Class::Unknown`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
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
            path: TypeVariantPath(`malamute::Class::Unknown`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::Class::Unknown`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
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
            path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Continue`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
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
            path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Continue`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
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
            path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Continue`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Continue`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field_ty_leash_class: Copyable,
            field: Tuple {
                index: 0,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
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
            path: TypeVariantPath(`core::ops::ControlFlow::Break`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Break`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
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
            path: TypeVariantPath(`core::ops::ControlFlow::Break`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Break`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
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
            path: TypeVariantPath(`core::ops::ControlFlow::Break`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Break`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`core::ops::ControlFlow::Break`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Break`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
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
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field_ty_leash_class: Other,
            field: Tuple {
                index: 0,
            },
        },
    },
]
```