[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`syntax_errors::ast::A`, `Struct`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructTypeDecTemplate {
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
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
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