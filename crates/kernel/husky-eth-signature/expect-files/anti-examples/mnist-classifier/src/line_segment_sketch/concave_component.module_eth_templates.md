```rust
[
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`ConcaveComponent`),
                                    ident: `line_segment_sketch`,
                                    ty: EthTerm(`Leash LineSegmentSketch`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`ConcaveComponent`),
                                    ident: `strokes`,
                                    ty: EthTerm(`Leash CyclicSlice LineSegmentStroke`),
                                },
                            ],
                            instance_constructor_ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`Leash LineSegmentSketch`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`Leash CyclicSlice LineSegmentStroke`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`ConcaveComponent`),
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
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`Leash LineSegmentSketch`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Vec ConcaveComponent`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Visualize`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 47,
                                            },
                                        ),
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
        ItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate {
                            path: TraitForTypeItemPath(
                                `<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConcaveComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConcaveComponent`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Visual`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`ConcaveComponent`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConcaveComponent`),
                            },
                            return_ty: EthTerm(`f32`),
                            expr_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConcaveComponent`),
                            },
                            return_ty: EthTerm(`f32`),
                            expr_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConcaveComponent`),
                            },
                            return_ty: EthTerm(`f32`),
                            expr_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConcaveComponent`),
                            },
                            return_ty: EthTerm(`f32`),
                            expr_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConcaveComponent`),
                            },
                            return_ty: EthTerm(`BoundingBox`),
                            expr_ty: EthTerm(`Leash BoundingBox`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConcaveComponent`),
                            },
                            return_ty: EthTerm(`RelativeBoundingBox`),
                            expr_ty: EthTerm(`Leash RelativeBoundingBox`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConcaveComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConcaveComponent`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`LineSegment`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConcaveComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConcaveComponent`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Point2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConcaveComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConcaveComponent`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Point2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConcaveComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConcaveComponent`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConcaveComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConcaveComponent`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConcaveComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConcaveComponent`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```