[
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`malamute::OneVsAll`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                        traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                        traits: [],
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
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`malamute::OneVsAllResult`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                        traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                        traits: [],
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
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`malamute::narrow_down`, `Gn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Gn(
                        GnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                        traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                        traits: [],
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
            TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(
                    Id {
                        value: 41,
                    },
                ),
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
                                            value: 5,
                                        },
                                    ),
                                    traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::ops::Unveil malamute::OneVsAllResult t a`),
                        self_ty: Path(
                            ExplicitApplication(
                                DeclarativeTermExplicitApplication(
                                    Id {
                                        value: 91,
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
                        ty_sketch: Path(
                            TypePath(
                                Id {
                                    value: 104,
                                },
                            ),
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