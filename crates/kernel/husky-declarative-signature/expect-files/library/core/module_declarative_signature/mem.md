[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::mem::Ref`, `Extern`),
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
                                                value: 7,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
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
                TypePath(`core::mem::RefMut`, `Extern`),
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
                                                value: 7,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Invariant,
                                        ),
                                        symbol: SymbolDeclarativeTerm(
                                            Id {
                                                value: 8,
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
                TypePath(`core::mem::Leash`, `Extern`),
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
                TypePath(`core::mem::At`, `Extern`),
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
                                        annotated_variance: None,
                                        symbol: SymbolDeclarativeTerm(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
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
                        module_path: `core::mem`,
                        trai_path: TraitPath(`core::marker::Copy`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::mem::Leash`, `Extern`),
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
                        trai: DeclarativeTerm(`core::marker::Copy`),
                        self_ty: DeclarativeSelfType::Path(
                            DeclarativeTerm(`core::mem::Leash t`),
                        ),
                    },
                ),
            ),
        ),
    ),
]