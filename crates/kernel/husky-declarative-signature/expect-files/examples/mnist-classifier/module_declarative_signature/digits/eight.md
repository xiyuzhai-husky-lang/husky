[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
            ),
        ),
        Err(
            DeclarativeSignatureError::DeclarativeTermError,
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
            ),
        ),
        Err(
            DeclarativeSignatureError::DeclarativeTermError,
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
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
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    Regular(
                                        ExplicitRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: ExplicitApplication(
                                                DeclarativeTermExplicitApplication(
                                                    Id {
                                                        value: 49,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
]