[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
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
                                ],
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    Regular(
                                        ExplicitRegularParameterDeclarativeSignatureTemplate {
                                            contract: Move,
                                            ty: ExplicitApplication(
                                                DeclarativeTermExplicitApplication(
                                                    Id {
                                                        value: 43,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::basic::unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
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
                                ],
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    Regular(
                                        ExplicitRegularParameterDeclarativeSignatureTemplate {
                                            contract: Move,
                                            ty: ExplicitApplication(
                                                DeclarativeTermExplicitApplication(
                                                    Id {
                                                        value: 43,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        ExplicitRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 54,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        ExplicitRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 54,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::basic::unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`quick_sort::partition`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
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
                                ],
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    Regular(
                                        ExplicitRegularParameterDeclarativeSignatureTemplate {
                                            contract: Move,
                                            ty: ExplicitApplication(
                                                DeclarativeTermExplicitApplication(
                                                    Id {
                                                        value: 43,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        ExplicitRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 54,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        ExplicitRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 54,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::isize`),
                        },
                    ),
                ),
            ),
        ),
    ),
]