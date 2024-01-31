[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`syntax_errors::ast::A`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`syntax_errors::ast::A`),
                            fields: [],
                            instance_constructor_ritchie_ty: RitchieDeclarativeTerm {
                                ritchie_kind: Type(
                                    Fn,
                                ),
                                params: [],
                                return_ty: DeclarativeTerm(`syntax_errors::ast::A`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `syntax_errors::ast`,
                                    ty_path: TypePath(`syntax_errors::ast::A`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
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
                        ty: DeclarativeTerm(`syntax_errors::ast::A`),
                    },
                ),
            ),
        ),
    ),
]