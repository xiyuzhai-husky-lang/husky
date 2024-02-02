[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::clone::Clone`),
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
                                    symbol: SymbolDecTerm(
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
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `core::clone`,
                        trai_path: TraitPath(`core::clone::Clone`),
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
                                    symbol: SymbolDecTerm(
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
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::DerivedAny(
                            SymbolDecTerm {
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
                                        CategoryTerm {
                                            universe: UniverseTerm(
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
                                            module_path: `core::clone`,
                                            trai_path: TraitPath(`core::clone::Clone`),
                                            ty_sketch: TypeSketch::DeriveAny,
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `clone`,
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
                                SymbolDecTerm(
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
                                    SymbolDecTerm(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: Symbol(
                                SymbolDecTerm(
                                    Id {
                                        value: 5,
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