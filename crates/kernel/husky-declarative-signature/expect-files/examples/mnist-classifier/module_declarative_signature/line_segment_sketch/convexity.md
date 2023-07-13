[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 90,
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
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::basic::bool`),
                        },
                    ),
                ),
            ),
        ),
    ),
]