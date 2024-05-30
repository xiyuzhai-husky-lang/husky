```rust
[
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(`mnist_classifier::line_segment_sketch::concave_component),
        ),
        Ok(
            ItemEthTemplate::Submodule(
                SubmoduleItemPath(`mnist_classifier::line_segment_sketch::concave_component),
            ),
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convex_component),
        ),
        Ok(
            ItemEthTemplate::Submodule(
                SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convex_component),
            ),
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convexity),
        ),
        Ok(
            ItemEthTemplate::Submodule(
                SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convexity),
            ),
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(`mnist_classifier::line_segment_sketch::line_segment),
        ),
        Ok(
            ItemEthTemplate::Submodule(
                SubmoduleItemPath(`mnist_classifier::line_segment_sketch::line_segment),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    ident: `points`,
                                    ty: EthTerm(`Leash CyclicSlice Point2d`),
                                },
                                PropsFieldEthTemplate {
                                    ident: `start`,
                                    ty: EthTerm(`Point2d`),
                                },
                                PropsFieldEthTemplate {
                                    ident: `end`,
                                    ty: EthTerm(`Point2d`),
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
                                            ty: EthTerm(`Leash CyclicSlice Point2d`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`LineSegmentStroke`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    ident: `contour`,
                                    ty: EthTerm(`Leash RawContour`),
                                },
                                PropsFieldEthTemplate {
                                    ident: `strokes`,
                                    ty: EthTerm(`Vec LineSegmentStroke`),
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
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Move,
                                            ty: EthTerm(`Vec LineSegmentStroke`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`LineSegmentSketch`),
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
                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
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
                                            ty: EthTerm(`Vector2d`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Vector2d`),
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
                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
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
                                            ty: EthTerm(`Vector2d`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Vector2d`),
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
                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
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
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`i32`),
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
                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
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
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`i32`),
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
                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
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
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Vec LineSegmentStroke`),
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
                TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)`),
            ),
        ),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)`),
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
                                                value: 40,
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
                    `<mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)>::visualize`,
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
                                value: 4,
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
                TypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)`),
            ),
        ),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`LineSegmentStroke`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`,
                    TypeItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    AssocRitchie(
                        TypeAssocRitchieEthTemplate(
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
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
                                value: 35,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)`),
            ),
        ),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)`),
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
                                                value: 41,
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
                    `<mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)>::visualize`,
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
                                value: 5,
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
                TypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)`),
            ),
        ),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`LineSegmentSketch`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`,
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
                                value: 18,
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
                    `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`,
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
                                value: 19,
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
                    `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`,
                    TypeItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    AssocRitchie(
                        TypeAssocRitchieEthTemplate(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
]
```