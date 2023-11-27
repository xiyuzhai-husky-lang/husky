[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::Slice`, `Extern`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Extern(
                        ExternTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
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
                TypePath(`core::slice::CyclicSlice`, `Extern`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Extern(
                        ExternTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
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
                    ItemPathId(
                        Id {
                            value: 107,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        ty: DeclarativeTerm(`core::slice::Slice t`),
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
                            value: 165,
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
                                        value: 165,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::slice::Slice t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::Slice t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::Slice t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::usize`),
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
                            value: 166,
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
                                        value: 166,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::slice::Slice t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::Slice t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: BorrowMut,
                                ty: DeclarativeTerm(`core::slice::Slice t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::usize`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::usize`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::basic::unit`),
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
                        module_path: `core::slice`,
                        trai_path: TraitPath(`core::ops::IntIndex`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::ops::IntIndex`),
                        self_ty: Path(
                            ExplicitApplication(
                                DeclarativeTermExplicitApplication(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
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
                            value: 167,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::AssociatedType(
                        TraitForTypeAssociatedTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            ty_term: DeclarativeTerm(`t`),
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
                    ItemPathId(
                        Id {
                            value: 109,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
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
                            value: 168,
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
                                        value: 168,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::i32`),
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
                            value: 169,
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
                                        value: 169,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::i32`),
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
                            value: 170,
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
                                        value: 170,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::i32`),
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
                            value: 171,
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
                                        value: 171,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option ~ t`),
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
                            value: 172,
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
                                        value: 172,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSlice t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option ~ t`),
                        },
                    ),
                ),
            ),
        ),
    ),
]