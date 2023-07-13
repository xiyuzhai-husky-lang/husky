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
                        generic_parameters: DeclarativeGenericParameters {
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
                        generic_parameters: DeclarativeGenericParameters {
                            data: [],
                        },
                    },
                ),
            ),
        ),
    ),
]