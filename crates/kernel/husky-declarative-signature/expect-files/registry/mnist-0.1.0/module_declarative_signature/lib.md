[
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist::MnistLabel`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist::MnistLabel`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist::BinaryImage28`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::TupleStruct(
                        TupleTypeStructDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist::BinaryImage28`),
                            fields: [
                                TupleStructFieldDeclarativeSignatureTemplate {
                                    ty: ExplicitApplication(
                                        DeclarativeTermExplicitApplication(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist::input`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`mnist::BinaryImage28`),
                        },
                    ),
                ),
            ),
        ),
    ),
]