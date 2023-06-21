[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Trait(
                TraitPath(`core::marker::Copy`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Trait(
                    TraitDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                    },
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Trait(
                TraitPath(`core::marker::Sized`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Trait(
                    TraitDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                    },
                ),
            ),
        ),
    ),
]