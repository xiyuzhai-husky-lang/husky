[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructTypeDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 49,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `line_segment_sketch`,
                                    ty: Application(
                                        DecApplication(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `line_segments`,
                                    ty: Application(
                                        DecApplication(
                                            Id {
                                                value: 29,
                                            },
                                        ),
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: Type(
                                    Fn,
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 27,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Move,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 29,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
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
                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
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
                            data: [],
                        },
                        trai: EntityPath(
                            Trait(
                                TraitPath(
                                    ItemPathId(
                                        Id {
                                            value: 181,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 49,
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
        ItemPath::AssocItem(
            AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId(
                        Id {
                            value: 352,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodFn(
                        TraitForTypeMethodFnDecTemplate {
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 49,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 144,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]