[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
            ),
        ),
        Err(
            DeclarativeSignatureError::DeclarativeTermError,
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
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
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
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
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
            ),
        ),
        Err(
            DeclarativeSignatureError::DeclarativeTermError,
        ),
    ),
]