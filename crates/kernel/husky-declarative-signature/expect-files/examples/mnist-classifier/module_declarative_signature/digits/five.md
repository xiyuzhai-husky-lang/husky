[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`malamute::OneVsAll mnist::MnistLabel mnist::MnistLabel::Five`),
                        },
                    ),
                ),
            ),
        ),
    ),
]