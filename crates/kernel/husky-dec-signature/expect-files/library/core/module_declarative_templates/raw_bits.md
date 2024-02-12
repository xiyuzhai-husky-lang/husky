[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::raw_bits::r32`, `Extern`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternTypeDecTemplate {
                            template_parameters: DecTemplateParameters {
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
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `core::raw_bits`,
                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
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
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: EntityPath(
                            Type(
                                TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 79,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::raw_bits::r32(0)::last_bits`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::raw_bits::r32(0)::last_bits`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 79,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 44,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 79,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::raw_bits::r32(0)::ctz`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::raw_bits::r32(0)::ctz`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 79,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::raw_bits::r32(0)::co`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::raw_bits::r32(0)::co`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 79,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::raw_bits::r32(0)::span`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::raw_bits::r32(0)::span`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 79,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::raw_bits::r32(0)::right_mass`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::raw_bits::r32(0)::right_mass`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            self_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 79,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]