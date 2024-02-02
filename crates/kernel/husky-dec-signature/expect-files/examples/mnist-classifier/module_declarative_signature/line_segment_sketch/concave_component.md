[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructTypeDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `line_segment_sketch`,
                                    ty: DecTerm(`~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `strokes`,
                                    ty: DecTerm(`~ core::slice::CyclicSlice mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: RitchieDecTerm {
                                ritchie_kind: Type(
                                    Fn,
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Move,
                                            ty: DecTerm(`~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Move,
                                            ty: DecTerm(`~ core::slice::CyclicSlice mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Fn(
                        MajorFnDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DecTerm(`~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm(`[] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
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
        Ok(
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        trai: DecTerm(`core::visual::Visualize`),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
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
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodFn(
                        TraitForTypeMethodFnDecTemplate {
                            self_ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`core::visual::Visual`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
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
        Ok(
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            return_ty: DecTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            return_ty: DecTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`, `MemoizedField`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`, `MemoizedField`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            return_ty: DecTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            return_ty: DecTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            return_ty: DecTerm(`mnist_classifier::geom2d::BoundingBox`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            return_ty: DecTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            self_ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: DecTerm(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
]