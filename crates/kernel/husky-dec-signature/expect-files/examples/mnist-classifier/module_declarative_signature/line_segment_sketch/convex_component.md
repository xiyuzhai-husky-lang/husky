[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `line_segment_sketch`,
                                    ty: DeclarativeTerm(`~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `line_segments`,
                                    ty: DeclarativeTerm(`~ core::slice::CyclicSlice mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: RitchieDeclarativeTerm {
                                ritchie_kind: Type(
                                    Fn,
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Move,
                                            ty: DeclarativeTerm(`~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Move,
                                            ty: DeclarativeTerm(`~ core::slice::CyclicSlice mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                                        },
                                    ),
                                ],
                                return_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`),
                            },
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
                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
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
                        trai: DeclarativeTerm(`core::visual::Visualize`),
                        self_ty: DeclarativeSelfType::Path(
                            DeclarativeTerm(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`),
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
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodFn(
                        TraitForTypeMethodFnDecTemplate {
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::visual::Visual`),
                        },
                    ),
                ),
            ),
        ),
    ),
]