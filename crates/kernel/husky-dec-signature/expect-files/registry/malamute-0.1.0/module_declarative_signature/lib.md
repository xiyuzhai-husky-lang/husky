[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::Class`, `Enum`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Enum(
                        EnumTypeDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDeclarativeTerm(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_ty: DeclarativeTerm(`malamute::Class t`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::OneVsAll`, `Enum`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Enum(
                        EnumTypeDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDeclarativeTerm(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDeclarativeTerm(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_ty: DeclarativeTerm(`malamute::OneVsAll t a`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::OneVsAllResult`, `Enum`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Enum(
                        EnumTypeDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDeclarativeTerm(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDeclarativeTerm(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_ty: DeclarativeTerm(`malamute::OneVsAllResult t a`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Gn(
                        MajorGnDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDeclarativeTerm(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDeclarativeTerm(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Variadic(
                                        DeclarativeRitchieVariadicParameter {
                                            contract: Pure,
                                            ty: DeclarativeTerm(`core::num::f32`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Keyed(
                                        DeclarativeRitchieKeyedParameter {
                                            key: `skip`,
                                            contract: Pure,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                            has_default: true,
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`malamute::OneVsAllResult t a`),
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
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::default::Default`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::OneVsAll`, `Enum`),
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
                                            value: 4,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: SymbolDeclarativeTerm(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::default::Default`),
                        self_ty: DeclarativeSelfType::Path(
                            DeclarativeTerm(`malamute::OneVsAll t a`),
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
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::default::Default`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `default`,
                                    item_kind: AssociatedFunctionFn,
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
                    TraitForTypeItemDecTemplate::AssociatedFn(
                        TraitForTypeAssociatedFnDecTemplate {
                            self_ty: DeclarativeTerm(`malamute::OneVsAll t a`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`malamute::OneVsAll t a`),
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
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::Class`, `Enum`),
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
                                            value: 3,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: SymbolDeclarativeTerm(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::ops::Unveil malamute::OneVsAll t a`),
                        self_ty: DeclarativeSelfType::Path(
                            DeclarativeTerm(`malamute::Class t`),
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
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::Class`, `Enum`),
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
                                                        module_path: `malamute`,
                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`malamute::Class`, `Enum`),
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
                            ty_term: DeclarativeTerm(`core::basic::unit`),
                        },
                    ),
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
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::Class`, `Enum`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `unveil`,
                                    item_kind: AssociatedFunctionFn,
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
                    TraitForTypeItemDecTemplate::AssociatedFn(
                        TraitForTypeAssociatedFnDecTemplate {
                            self_ty: DeclarativeTerm(`malamute::Class t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DeclarativeTerm(`malamute::OneVsAll t a`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::ops::ControlFlow malamute::Class t(`),
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
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::OneVsAll`, `Enum`),
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
                                            value: 4,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: SymbolDeclarativeTerm(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::ops::Unveil malamute::OneVsAllResult t a`),
                        self_ty: DeclarativeSelfType::Path(
                            DeclarativeTerm(`malamute::OneVsAll t a`),
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
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                        module_path: `malamute`,
                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
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
                            ty_term: DeclarativeTerm(`core::basic::unit`),
                        },
                    ),
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
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `unveil`,
                                    item_kind: AssociatedFunctionFn,
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
                    TraitForTypeItemDecTemplate::AssociatedFn(
                        TraitForTypeAssociatedFnDecTemplate {
                            self_ty: DeclarativeTerm(`malamute::OneVsAll t a`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DeclarativeTerm(`malamute::OneVsAllResult t a`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::ops::ControlFlow malamute::OneVsAll t a(`),
                        },
                    ),
                ),
            ),
        ),
    ),
]