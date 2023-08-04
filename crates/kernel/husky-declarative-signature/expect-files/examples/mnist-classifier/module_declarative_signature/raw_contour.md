[
    (
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `cc`,
                                    ty: DeclarativeTerm(`~ mnist_classifier::connected_component::ConnectedComponent`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `points`,
                                    ty: DeclarativeTerm(`[] mnist_classifier::geom2d::Point2d`),
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::raw_contour::Direction`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::raw_bits::r32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::raw_bits::r32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::raw_bits::r32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::raw_bits::r32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::raw_bits::r32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::raw_bits::r32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::raw_bits::r32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::raw_bits::r32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::raw_contour::Direction`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::raw_contour::Direction`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::raw_contour::Direction`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::raw_bits::r32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::raw_bits::r32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::raw_contour::Direction`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::raw_contour::Direction`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::raw_contour::StreakCache`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `prev1`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `prev2`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`[] mnist_classifier::geom2d::Point2d`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`~ mnist_classifier::connected_component::ConnectedComponent`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`[] mnist_classifier::raw_contour::RawContour`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
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
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        trai: DeclarativeTerm(`core::visual::Visualize`),
                        self_ty: Path(
                            EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 82,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
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
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                        TraitForTypeMethodFnDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::visual::Html`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath {
                    module_path: `mnist_classifier::raw_contour`,
                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    disambiguator: 0,
                },
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                    },
                ),
            ),
        ),
    ),
    (
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
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
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
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
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
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
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
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
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
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
]