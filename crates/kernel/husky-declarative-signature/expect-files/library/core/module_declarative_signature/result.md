[
    Ok(
        SignatureTemplate::Type(
            TypeDeclarativeSignatureTemplate::Enum(
                EnumDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                traits: [],
                            },
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                TraitForTypeImplBlockDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                traits: [],
                            },
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                traits: [],
                            },
                            ImplicitParameterDeclarativeSignature {
                                annotated_variance: None,
                                symbol: DeclarativeTermSymbol(
                                    Id {
                                        value: 26,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                    trai: DeclarativeTerm(`core::ops::Unveil(core::result::Result t s, `),
                    ty: DeclarativeTerm(`core::result::Result t s`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                TraitForTypeItemDeclarativeSignatureTemplate::AssociatedType(
                    TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
                ),
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                    TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                            contract: Pure,
                            ty: ExplicitApplication(
                                DeclarativeTermExplicitApplication(
                                    Id {
                                        value: 35,
                                    },
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            data: [
                                ExplicitParameterDeclarativeSignatureTemplate {
                                    contract: Pure,
                                    ty: ExplicitApplication(
                                        DeclarativeTermExplicitApplication(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: DeclarativeTerm(`core::result::Result t s`),
                    },
                ),
            ),
        ),
    ),
]