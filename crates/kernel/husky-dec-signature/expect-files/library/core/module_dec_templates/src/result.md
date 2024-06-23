```rust
[
    (
        ItemPath(`core::result::Result`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Enum(
                        EnumDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 44,
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
        ItemPath(`core::result::Result as core::ops::Unveil(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 12,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: Application(
                            DecApplication(
                                Id {
                                    value: 47,
                                },
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            Application(
                                DecApplication(
                                    Id {
                                        value: 48,
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
        ItemPath(`<core::result::Result as core::ops::Unveil(0)>::Continue`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocType(
                        TraitForTypeAssocTypeDecTemplate {
                            path: TraitForTypeItemPath(
                                `<core::result::Result as core::ops::Unveil(0)>::Continue`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            ty_term: SymbolicVariable(
                                DecSymbolicVariable(
                                    Id {
                                        value: 13,
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
        ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocRitchie(
                        TraitForTypeAssocRitchieDecTemplate {
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
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
                                                        value: 50,
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
                                        value: 48,
                                    },
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