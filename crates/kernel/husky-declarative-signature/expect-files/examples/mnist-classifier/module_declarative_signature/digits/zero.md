[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`mnist_classifier::fermi::FermiMatchResult`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
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
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: ExplicitApplication(
                                            DeclarativeTermExplicitApplication(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    },
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
                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`core::option::Option mnist::MnistLabel`),
                        },
                    ),
                ),
            ),
        ),
    ),
]