[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::result::Result`, `Enum`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Enum(
                        EnumTypeDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSvar(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSvar(
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
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `core::result`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::result::Result`, `Enum`),
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
                        template_parameters: DecTemplateParameters {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSvar(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSvar(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSvar(
                                        Id {
                                            value: 12,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSvar(
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
                                    value: 33,
                                },
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            Application(
                                DecApplication(
                                    Id {
                                        value: 34,
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
        ItemPath::AssocItem(
            AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssocItem(
                            AssocItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `core::result`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`core::result::Result`, `Enum`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `Continue`,
                                    item_kind: AssocType,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocType(
                        TraitForTypeAssocTypeDecTemplate {
                            path: TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssocItem(
                                        AssocItemPathData::TraitForTypeItem(
                                            TraitForTypeItemPathData {
                                                impl_block: TraitForTypeImplBlock {
                                                    data: TraitForTypeImplBlockPathData {
                                                        module_path: `core::result`,
                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`core::result::Result`, `Enum`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                                ident: `Continue`,
                                                item_kind: AssocType,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            ty_term: Symbol(
                                DecSvar(
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
        ItemPath::AssocItem(
            AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssocItem(
                            AssocItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `core::result`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`core::result::Result`, `Enum`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `unveil`,
                                    item_kind: AssocFunctionFn,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocFn(
                        TraitForTypeAssocFnDecTemplate {
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 34,
                                    },
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 36,
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
                                        value: 34,
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