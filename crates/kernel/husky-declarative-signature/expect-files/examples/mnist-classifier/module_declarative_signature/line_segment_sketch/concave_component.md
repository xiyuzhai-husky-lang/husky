[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `line_segment_sketch`,
                                    ty: DeclarativeTerm(`~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `strokes`,
                                    ty: DeclarativeTerm(`~ core::slice::CyclicSlice mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                                    has_initialization: false,
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
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
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
                                            ty: DeclarativeTerm(`~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                            value: 57,
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `norm`,
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `rel_norm`,
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `hausdorff_norm`,
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `angle_change`,
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `line_segment`,
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `start`,
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `end`,
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `start_tangent`,
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `end_tangent`,
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
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
]