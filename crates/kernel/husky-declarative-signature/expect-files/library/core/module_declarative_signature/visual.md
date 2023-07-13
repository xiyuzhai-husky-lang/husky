[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Trait(
                TraitPath(`core::visual::Visualize`),
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
            ModuleItemPath::Type(
                TypePath(`core::visual::Html`, `Extern`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Extern(
                        ExternDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
]