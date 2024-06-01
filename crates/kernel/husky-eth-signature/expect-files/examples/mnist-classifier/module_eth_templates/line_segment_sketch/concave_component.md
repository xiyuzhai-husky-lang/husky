```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
            ),
        ),
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
                                            contract: Move,
                                            ty: EthTerm(`Leash LineSegmentSketch`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Move,
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
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
                                            contract: Pure,
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
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
            ),
        ),
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
                            EntityPath(
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
        ItemPath::AssocItem(
            AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`,
                    TraitItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
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
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
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