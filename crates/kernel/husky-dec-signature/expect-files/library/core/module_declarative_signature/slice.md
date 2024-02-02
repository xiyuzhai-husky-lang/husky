[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::Slice`, `Extern`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternTypeDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        symbol: SymbolDeclarativeTerm(
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
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternTypeDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        symbol: SymbolDeclarativeTerm(
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
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: SymbolDeclarativeTerm(
                                        Id {
                                            value: 10,
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
                TypeItemPath(`(core::slice::Slice(0)::len`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::slice::Slice(0)::len`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDeclarativeTerm(
                                                Id {
                                                    value: 10,
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
                                contract: Pure,
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
                TypeItemPath(`(core::slice::Slice(0)::swap`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::slice::Slice(0)::swap`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDeclarativeTerm(
                                                Id {
                                                    value: 10,
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
                                        symbol: SymbolDeclarativeTerm(
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
                                            contract: Pure,
                                            ty: DeclarativeTerm(`core::num::usize`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
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
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: SymbolDeclarativeTerm(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::ops::IntIndex`),
                        self_ty: DeclarativeSelfType::Path(
                            DeclarativeTerm(`core::slice::CyclicSlice t`),
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
                                            module_path: `core::slice`,
                                            trai_path: TraitPath(`core::ops::IntIndex`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `Output`,
                                    item_kind: AssociatedType,
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
                    TraitForTypeItemDecTemplate::AssociatedType(
                        TraitForTypeAssociatedTypeDecTemplate {
                            path: TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
                                            TraitForTypeItemPathData {
                                                impl_block: TraitForTypeImplBlock {
                                                    data: TraitForTypeImplBlockPathData {
                                                        module_path: `core::slice`,
                                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                                ident: `Output`,
                                                item_kind: AssociatedType,
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: SymbolDeclarativeTerm(
                                        Id {
                                            value: 10,
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
                TypeItemPath(`(core::slice::CyclicSlice(0)::ilen`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::slice::CyclicSlice(0)::ilen`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDeclarativeTerm(
                                                Id {
                                                    value: 10,
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
                                contract: Pure,
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
                TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDeclarativeTerm(
                                                Id {
                                                    value: 10,
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
                                contract: Pure,
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
                TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDeclarativeTerm(
                                                Id {
                                                    value: 10,
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
                                contract: Pure,
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
                TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDeclarativeTerm(
                                                Id {
                                                    value: 10,
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
                                contract: Pure,
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
                TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDeclarativeTerm(
                                                Id {
                                                    value: 10,
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
                                contract: Pure,
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