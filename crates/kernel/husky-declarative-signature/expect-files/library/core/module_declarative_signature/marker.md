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
                        generic_parameters: DeclarativeGenericParameterTemplates {
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
                        generic_parameters: DeclarativeGenericParameterTemplates {
                            data: [],
                        },
                    },
                ),
            ),
        ),
    ),
]