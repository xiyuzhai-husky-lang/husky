[
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`natural_number_game::Nat`, `Inductive`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Inductive(
                        InductiveTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`natural_number_game::OddNat`, `Structure`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Structure(
                        StructureTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`natural_number_game::EvenNat`, `Structure`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Structure(
                        StructureTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            TypeImplBlock(
                TypeImplBlockPath(
                    Id {
                        value: 47,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`natural_number_game::Nat`),
                    },
                ),
            ),
        ),
    ),
]