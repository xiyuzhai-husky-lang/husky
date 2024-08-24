```rust
[
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    self_ty: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                    ident: `line_segment_sketch`,
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
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    self_ty: DecTerm::ItemPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                    ident: `strokes`,
                                    ty: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::LeashOrBitNot(
                                                Toolchain {
                                                    data: ToolchainData::Local {
                                                        library_path: "../../../library",
                                                    },
                                                },
                                            ),
                                            argument: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::ItemPath(
                                                        DecItemPath::Type(
                                                            TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        ),
                                                    ),
                                                    argument: DecTerm::ItemPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        ),
                                                    ),
                                                },
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::Application(
                                                        DecApplication {
                                                            function: DecTerm::ItemPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                ),
                                                            ),
                                                            argument: DecTerm::ItemPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
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
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::ItemPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`,
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`,
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
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
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
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
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`,
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
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
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`,
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
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