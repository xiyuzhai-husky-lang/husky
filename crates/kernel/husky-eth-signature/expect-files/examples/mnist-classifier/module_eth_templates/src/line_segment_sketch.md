```rust
[
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component`),
        Ok(
            ItemEthTemplate::Submodule(
                SubmoduleItemPath(`mnist_classifier::line_segment_sketch::concave_component),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::convex_component`),
        Ok(
            ItemEthTemplate::Submodule(
                SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convex_component),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::convexity`),
        Ok(
            ItemEthTemplate::Submodule(
                SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convexity),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::line_segment`),
        Ok(
            ItemEthTemplate::Submodule(
                SubmoduleItemPath(`mnist_classifier::line_segment_sketch::line_segment),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
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
                                    self_ty: EthTerm(`LineSegmentStroke`),
                                    ident: `points`,
                                    ty: EthTerm(`Leash CyclicSlice Point2d`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`LineSegmentStroke`),
                                    ident: `start`,
                                    ty: EthTerm(`Point2d`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`LineSegmentStroke`),
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
                                            contract: Contract::Move,
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
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
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
                                    self_ty: EthTerm(`LineSegmentSketch`),
                                    ident: `contour`,
                                    ty: EthTerm(`Leash RawContour`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`LineSegmentSketch`),
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
                                            contract: Contract::Move,
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
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
        ItemPath(`mnist_classifier::line_segment_sketch::go_right`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
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
                                            ty: EthTerm(`Vector2d`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
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
        ItemPath(`mnist_classifier::line_segment_sketch::go_left`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
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
                                            ty: EthTerm(`Vector2d`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
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
        ItemPath(`mnist_classifier::line_segment_sketch::extend_end`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
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
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
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
        ItemPath(`mnist_classifier::line_segment_sketch::extend_start`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
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
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
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
        ItemPath(`mnist_classifier::line_segment_sketch::find_line_segments`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
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
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
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
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)`),
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
                            ItemPath(
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
        ItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate {
                            path: TraitForTypeItemPath(
                                `<mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)>::visualize`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`LineSegmentStroke`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`LineSegmentStroke`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::AssocRitchie(
                        TypeAssocRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`,
                                TypeItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`LineSegmentStroke`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`LineSegmentStroke`),
                            ty: EthTerm(`fn(( Leash RawContour,  i32,  i32) -> LineSegmentStroke`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`LineSegmentStroke`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`LineSegmentStroke`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)`),
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
                            ItemPath(
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
        ItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate {
                            path: TraitForTypeItemPath(
                                `<mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)>::visualize`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`LineSegmentSketch`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`LineSegmentSketch`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`LineSegmentSketch`),
                            },
                            return_ty: EthTerm(`Vec ConcaveComponent`),
                            expr_ty: EthTerm(`Leash Vec ConcaveComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`LineSegmentSketch`),
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
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::AssocRitchie(
                        TypeAssocRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`,
                                TypeItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`LineSegmentSketch`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`Leash RawContour`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`LineSegmentSketch`),
                            ty: EthTerm(`fn(( Leash RawContour,  f32) -> LineSegmentSketch`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```