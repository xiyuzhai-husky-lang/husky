```rust
[
    (
        ItemPath(`mnist_classifier::raw_contour::RawContour`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    self_ty: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                    ),
                                    ident: `cc`,
                                    ty: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::LeashOrBitNot(
                                                Toolchain {
                                                    data: ToolchainData::Local {
                                                        library_path: "../../../library",
                                                    },
                                                },
                                            ),
                                            argument: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    self_ty: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                    ),
                                    ident: `points`,
                                    ty: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::List(
                                                DecList {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    items: [],
                                                },
                                            ),
                                            argument: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::ItemPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::List(
                                                        DecList {
                                                            toolchain: Toolchain {
                                                                data: ToolchainData::Local {
                                                                    library_path: "../../../library",
                                                                },
                                                            },
                                                            items: [],
                                                        },
                                                    ),
                                                    argument: DecTerm::ItemPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::Direction`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Enum(
                        EnumDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::get_pixel_pair`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::get_inward_direction`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::get_angle_change`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::get_outward_direction`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::StreakCache`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    self_ty: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                        ),
                                    ),
                                    ident: `prev1`,
                                    ty: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::i32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    self_ty: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                        ),
                                    ),
                                    ident: `prev2`,
                                    ty: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`core::num::i32`, `Extern`),
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::get_concave_middle_point`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::List(
                                                        DecList {
                                                            toolchain: Toolchain {
                                                                data: ToolchainData::Local {
                                                                    library_path: "../../../library",
                                                                },
                                                            },
                                                            items: [],
                                                        },
                                                    ),
                                                    argument: DecTerm::ItemPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::find_raw_contours`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::ItemPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::List(
                                        DecList {
                                            toolchain: Toolchain {
                                                data: ToolchainData::Local {
                                                    library_path: "../../../library",
                                                },
                                            },
                                            items: [],
                                        },
                                    ),
                                    argument: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::ItemPath(
                            DecItemPath::Trait(
                                TraitPath(`core::visual::Visualize`),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`core::visual::Visual`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::RawContour(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::ItemPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::RawContour(0)::bounding_box`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::RawContour(0)::contour_len`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::raw_contour::RawContour(0)::contour_len`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::raw_contour::RawContour(0)::displacement`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::ItemPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```