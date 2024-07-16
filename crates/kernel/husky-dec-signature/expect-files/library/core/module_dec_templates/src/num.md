```rust
[
    (
        ItemPath(`core::num::i8`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::i16`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::i32`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::i64`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::i128`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::isize`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::u8`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::u16`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::u32`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::u64`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::u128`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::usize`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::f32`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::f64`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternDecTemplate {
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
        ItemPath(`core::num::i8(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::i8`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i8(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::i8(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i8`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i8`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i8(0)::max`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::i8(0)::max`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i8`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i8`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i8`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i8 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i8`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::i8 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i8`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i8`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i16(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::i16`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i16(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::i16(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i16`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i16`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i16(0)::max`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::i16(0)::max`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i16`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i16`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i16`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i16 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::i16 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i16`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::i32(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32(0)::max`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::i32(0)::max`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32(0)::min`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::i32(0)::min`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::i32 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i64(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::i64`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i64(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::i64(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i64`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i64`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i64`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i64`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i64 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i64`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i64`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::i64 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i64`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i64`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i64`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i64`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i128(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::i128`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i128(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::i128(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i128`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i128`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i128`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i128`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i128 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i128`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i128`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::i128 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i128`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::i128`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i128`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i128`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::isize(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::isize(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::isize(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::isize`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::isize`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::isize as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::isize`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::isize as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::isize`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::isize`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::isize`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u8(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::u8`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u8(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::u8(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u8`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u8`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u8`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u8`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u8 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u8`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u8`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::u8 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u8`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u8`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::u8`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u8`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u16(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::u16`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u16(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::u16(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u16`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u16`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u16`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u16`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u16 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u16`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u16`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::u16 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u16`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u16`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::u16`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u16`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u32(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::u32`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u32(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::u32(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u32 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u32`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u32`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::u32 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::u32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u64(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::u64`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u64(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::u64(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u64`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u64`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u64`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u64`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u64 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u64`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u64`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::u64 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u64`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u64`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::u64`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u64`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u128(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::u128`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u128(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::u128(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u128`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u128`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u128`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u128`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u128 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u128`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u128`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::u128 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u128`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::u128`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::u128`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::u128`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::usize(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::usize(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::usize(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::usize`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::usize`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::usize`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::usize`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::usize as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::usize`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::usize`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::usize as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::usize`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::usize`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::usize`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f32(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::sqrt`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f32(0)::sqrt`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::max`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f32(0)::max`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::min`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f32(0)::min`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::sgnx`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f32(0)::sgnx`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::cos`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f32(0)::cos`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::sin`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f32(0)::sin`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::acos`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f32(0)::acos`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::f32 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f64(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`core::num::f64`, `Extern`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f64(0)::abs`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f64(0)::abs`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f64`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f64`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f64(0)::acos`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `core::num::f64(0)::acos`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f64`, `Extern`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f64`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f64 as core::ops::Add(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::Application(
                            DecApplication {
                                function: DecTerm::EntityPath(
                                    DecItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                ),
                                argument: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f64`, `Extern`),
                                    ),
                                ),
                            },
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<core::num::f64 as core::ops::Add(0)>::add`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`core::num::f64`, `Extern`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f64`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```