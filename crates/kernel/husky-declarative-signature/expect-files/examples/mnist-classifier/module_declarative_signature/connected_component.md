[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `row_start`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `row_end`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `upper_mass`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `lower_mass`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::EffHoles`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `matches`,
                                    ty: DeclarativeTerm(`[] core::option::Option ~ mnist_classifier::raw_contour::RawContour`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::FunctionFn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `mask`,
                                    ty: DeclarativeTerm(`mnist::BinaryImage28`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::FunctionFn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::raw_bits::r32`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::raw_bits::r32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::raw_bits::r32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::FunctionFn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist::BinaryImage28`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`[] mnist_classifier::connected_component::ConnectedComponent`),
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
                        module_path: `mnist_classifier::connected_component`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        ),
                        disambiguator: 0,
                    },
                },
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        trai: DeclarativeTerm(`core::visual::Visualize`),
                        self_ty: DeclarativeSelfType::Path(
                            DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
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
                    ItemPathId(
                        Id {
                            value: 376,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                        TraitForTypeMethodFnDeclarativeSignatureTemplate {
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::visual::Html`),
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
                                    module_path: `mnist_classifier::connected_component`,
                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 377,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 377,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`[] mnist_classifier::raw_contour::RawContour`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 378,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 378,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::connected_component::EffHoles`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 379,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 379,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 380,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 380,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 381,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 381,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 382,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 382,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 383,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 383,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 384,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 384,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 385,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 385,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 386,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 386,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
]