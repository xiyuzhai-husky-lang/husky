[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::Class`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumTypeDeclarativeSignatureTemplate {
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
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumTypeDeclarativeSignatureTemplate {
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
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 14,
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
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumTypeDeclarativeSignatureTemplate {
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
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 14,
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
                FugitivePath(`malamute::narrow_down`, `Gn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Gn(
                        GnFugitiveDeclarativeSignatureTemplate {
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
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 14,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Variadic(
                                        DeclarativeTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::f32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Keyed(
                                        DeclarativeTermRitchieKeyedParameter {
                                            key: `skip`,
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                            default: Some(
                                                DeclarativeTerm(`DeclarativeTermLiteralTodo`),
                                            ),
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
                TraitForTypeImplBlockPath {
                    module_path: `malamute`,
                    trai_path: TraitPath(`core::ops::Unveil`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                    disambiguator: 0,
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
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 14,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::ops::Unveil malamute::OneVsAll t a`),
                        self_ty: Path(
                            ExplicitApplication(
                                DeclarativeTermExplicitApplication(
                                    Id {
                                        value: 95,
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
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::Class`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                    ident: `Output`,
                    item_kind: AssociatedType,
                },
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
                            ty_term: DeclarativeTerm(`core::basic::unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
                    module_path: `malamute`,
                    trai_path: TraitPath(`core::ops::Unveil`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                    disambiguator: 0,
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
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 14,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::ops::Unveil malamute::OneVsAllResult t a`),
                        self_ty: Path(
                            ExplicitApplication(
                                DeclarativeTermExplicitApplication(
                                    Id {
                                        value: 97,
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
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::OneVsAll`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                    ident: `Output`,
                    item_kind: AssociatedType,
                },
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
                            ty_term: DeclarativeTerm(`core::basic::unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
]