[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`malamute::OneVsAll`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
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
                                                value: 49,
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
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`malamute::OneVsAllResult`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
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
                                                value: 49,
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
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`malamute::narrow_down`, `Gn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Gn(
                        GnDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
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
                                                value: 49,
                                            },
                                        ),
                                        traits: [],
                                    },
                                ],
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Variadic(
                                        SpecificVariadicParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        SpecificKeyedParameterDeclarativeSignatureTemplate {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 447,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    Unresolved(
                                                        RegularInteger(
                                                            5,
                                                        ),
                                                    ),
                                                ),
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
        EntityPath::ImplBlock(
            TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(
                    Id {
                        value: 39,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDeclarativeSignatureTemplate {
                        generic_parameters: DeclarativeGenericParameters {
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
                                            value: 49,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                        trai: DeclarativeTerm(`core::ops::Unveil malamute::OneVsAllResult t a`),
                        ty: DeclarativeTerm(`malamute::OneVsAll t a`),
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
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
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
                            generic_parameters: DeclarativeGenericParameters {
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