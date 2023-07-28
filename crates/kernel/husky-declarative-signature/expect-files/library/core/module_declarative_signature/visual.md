[
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Trait(
                TraitPath(`core::visual::Visualize`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Trait(
                    TraitDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::visual::Html`, `Extern`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Extern(
                        ExternDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
]