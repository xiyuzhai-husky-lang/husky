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
                                ],
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: Move,
                                            ty: ExplicitApplication(
                                                DeclarativeTermExplicitApplication(
                                                    Id {
                                                        value: 44,
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
                                ],
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: Move,
                                            ty: ExplicitApplication(
                                                DeclarativeTermExplicitApplication(
                                                    Id {
                                                        value: 44,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 45,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 45,
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
                                ],
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: Move,
                                            ty: ExplicitApplication(
                                                DeclarativeTermExplicitApplication(
                                                    Id {
                                                        value: 44,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 45,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 45,
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