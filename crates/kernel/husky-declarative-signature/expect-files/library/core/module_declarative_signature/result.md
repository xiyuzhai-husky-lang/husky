[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::result::Result`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [
                                    DeclarativeGenericParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                        traits: [],
                                    },
                                    DeclarativeGenericParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                        traits: [],
                                    },
                                ],
                            },
                            self_ty: DeclarativeTerm(`core::result::Result t s`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ImplBlock(
            TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(
                    Id {
                        value: 33,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDeclarativeSignatureTemplate {
                        generic_parameters: DeclarativeGenericParameterTemplates {
                            data: [
                                DeclarativeGenericParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                    traits: [],
                                },
                                DeclarativeGenericParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                    traits: [],
                                },
                                DeclarativeGenericParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 26,
                                        },
                                    ),
                                    traits: [],
                                },
                                DeclarativeGenericParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::ops::Unveil core::result::Result t s`),
                        self_ty: Path(
                            ExplicitApplication(
                                DeclarativeTermExplicitApplication(
                                    Id {
                                        value: 38,
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
        EntityPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `core::result`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: Path(
                            TypePath(
                                Id {
                                    value: 31,
                                },
                            ),
                        ),
                        disambiguator: 0,
                    },
                    ident: `Continue`,
                    item_kind: AssociatedType,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::AssociatedType(
                        TraitForTypeAssociatedTypeDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameterTemplates {
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
        EntityPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `core::result`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: Path(
                            TypePath(
                                Id {
                                    value: 31,
                                },
                            ),
                        ),
                        disambiguator: 0,
                    },
                    ident: `branch`,
                    item_kind: MethodFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                        TraitForTypeMethodFnDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: None,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                            parenic_parameters: DeclarativeParenicParameters {
                                data: [
                                    Regular(
                                        SpecificRegularDeclarativeParameterTemplate {
                                            contract: None,
                                            ty: ExplicitApplication(
                                                DeclarativeTermExplicitApplication(
                                                    Id {
                                                        value: 40,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::result::Result t s`),
                        },
                    ),
                ),
            ),
        ),
    ),
]