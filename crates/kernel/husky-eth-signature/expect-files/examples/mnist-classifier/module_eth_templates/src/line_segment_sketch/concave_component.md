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
                                    ident: `line_segment_sketch`,
                                    ty: EthTerm(`Leash LineSegmentSketch`),
                                },
                                PropsFieldEthTemplate {
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
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 2,
                            },
                        ),
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
                    MemoizedField(
                        TypeMemoizedFieldEthTemplate(
                            Id {
                                value: 12,
                            },
                        ),
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
                    MemoizedField(
                        TypeMemoizedFieldEthTemplate(
                            Id {
                                value: 13,
                            },
                        ),
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
                    MemoizedField(
                        TypeMemoizedFieldEthTemplate(
                            Id {
                                value: 14,
                            },
                        ),
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
                    MemoizedField(
                        TypeMemoizedFieldEthTemplate(
                            Id {
                                value: 15,
                            },
                        ),
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
                    MemoizedField(
                        TypeMemoizedFieldEthTemplate(
                            Id {
                                value: 16,
                            },
                        ),
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
                    MemoizedField(
                        TypeMemoizedFieldEthTemplate(
                            Id {
                                value: 17,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 27,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 28,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 29,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 30,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 31,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
]
```