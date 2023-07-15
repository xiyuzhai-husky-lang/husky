[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist::MnistLabel`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
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
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist::BinaryImage28`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::TupleStruct(
                        TupleStructDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist::BinaryImage28`),
                            fields: [
                                TupleStructFieldDeclarativeSignatureTemplate {
                                    ty: ExplicitApplication(
                                        DeclarativeTermExplicitApplication(
                                            Id {
                                                value: 91,
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
        EntityPath::ModuleItem(
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