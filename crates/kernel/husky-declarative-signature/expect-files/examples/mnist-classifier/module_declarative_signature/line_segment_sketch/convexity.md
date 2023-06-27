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
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
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
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 51,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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