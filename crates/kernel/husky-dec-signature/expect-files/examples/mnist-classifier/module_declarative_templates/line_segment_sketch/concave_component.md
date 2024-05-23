```rust
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
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `line_segment_sketch`,
                                    ty: Application(
                                        DecApplication(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `strokes`,
                                    ty: Application(
                                        DecApplication(
                                            Id {
                                                value: 29,
                                            },
                                        ),
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
                                            contract: Move,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 27,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 29,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
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
                                            contract: Pure,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 27,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 30,
                                    },
                                ),
                            ),
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
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: EntityPath(
                            Trait(
                                TraitPath(
                                    ItemPathId(
                                        Id {
                                            value: 180,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            EntityPath(
                                Type(
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
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 153,
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
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
            ),
        ),
        Ok(
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: EntityPath(
                            Type(
                                TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 47,
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
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 137,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 137,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 137,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 137,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 34,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 35,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 51,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 30,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 30,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 32,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 32,
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
            DecTemplate::AssocItem(
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
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 32,
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
    ),
]
```