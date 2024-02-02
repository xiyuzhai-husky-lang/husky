[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::visual::Visualize`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Trait(
                    TraitDecTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DecSymbol(
                                        Id {
                                            value: 5,
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
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::visual::Visual`, `Extern`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternTypeDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
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
                        module_path: `core::visual`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::DeriveAny,
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
                                    symbol: DecSymbol(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: EntityPath(
                            Trait(
                                TraitPath(
                                    ItemPathId(
                                        Id {
                                            value: 88,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::DerivedAny(
                            DecSymbol {
                                toolchain: Toolchain {
                                    data: ToolchainData::Local {
                                        library_path: VirtualPath {
                                            _data: VirtualPathBuf(
                                                "../../../library",
                                            ),
                                        },
                                    },
                                },
                                ty: Ok(
                                    Category(
                                        Category {
                                            universe: Universe(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                                index: DecTermSymbolIndex(
                                    SelfType,
                                ),
                            },
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
                                            module_path: `core::visual`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::DeriveAny,
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `visualize`,
                                    item_kind: MethodFn,
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
                    TraitForTypeItemDecTemplate::MethodFn(
                        TraitForTypeMethodFnDecTemplate {
                            self_ty: Symbol(
                                DecSymbol(
                                    Id {
                                        value: 5,
                                    },
                                ),
                            ),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: Symbol(
                                    DecSymbol(
                                        Id {
                                            value: 5,
                                        },
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
                                                value: 89,
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