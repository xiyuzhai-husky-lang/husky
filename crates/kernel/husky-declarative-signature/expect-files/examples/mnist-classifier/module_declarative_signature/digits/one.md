[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
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
                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
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