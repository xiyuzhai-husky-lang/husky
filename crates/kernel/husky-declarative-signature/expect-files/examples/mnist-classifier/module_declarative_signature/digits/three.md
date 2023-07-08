[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
            ),
        ),
        Err(
            DeclarativeSignatureError::DeclarativeTermError,
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
            ),
        ),
        Err(
            DeclarativeSignatureError::DeclarativeTermError,
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
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
                FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
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
                FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
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