```rust
[
    (
        ItemPath(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`ConnectedComponentDistribution`),
                                    ident: `row_start`,
                                    ty: EthTerm(`i32`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`ConnectedComponentDistribution`),
                                    ident: `row_end`,
                                    ty: EthTerm(`i32`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`ConnectedComponentDistribution`),
                                    ident: `upper_mass`,
                                    ty: EthTerm(`i32`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`ConnectedComponentDistribution`),
                                    ident: `lower_mass`,
                                    ty: EthTerm(`i32`),
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
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`ConnectedComponentDistribution`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::EffHoles`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`EffHoles`),
                                    ident: `matches`,
                                    ty: EthTerm(`Vec Option Leash RawContour`),
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
                                            ty: EthTerm(`Vec Option Leash RawContour`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`EffHoles`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::hole_tmpl`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
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
                                ],
                                return_ty: EthTerm(`Option f32`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`ConnectedComponent`),
                                    ident: `mask`,
                                    ty: EthTerm(`BinaryImage28`),
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
                                            ty: EthTerm(`BinaryImage28`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`ConnectedComponent`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
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
                                            ty: EthTerm(`r32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`r32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`r32`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::find_connected_components`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
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
                                            ty: EthTerm(`BinaryImage28`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Vec ConnectedComponent`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
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
                                                value: 12,
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
        ItemPath(`<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate {
                            path: TraitForTypeItemPath(
                                `<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConnectedComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConnectedComponent`),
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
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`ConnectedComponent`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConnectedComponent`),
                            },
                            return_ty: EthTerm(`Vec RawContour`),
                            expr_ty: EthTerm(`Leash Vec RawContour`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConnectedComponent`),
                            },
                            return_ty: EthTerm(`EffHoles`),
                            expr_ty: EthTerm(`Leash EffHoles`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConnectedComponent`),
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
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConnectedComponent`),
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
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConnectedComponent`),
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
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::distribution`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::distribution`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConnectedComponent`),
                            },
                            return_ty: EthTerm(`ConnectedComponentDistribution`),
                            expr_ty: EthTerm(`Leash ConnectedComponentDistribution`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConnectedComponent`),
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
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`ConnectedComponent`),
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
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConnectedComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConnectedComponent`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`ConnectedComponent`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`ConnectedComponent`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```