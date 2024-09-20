```rust
[
    Linket {
        data: LinketData::EnumUnitToJsonValue {
            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAll::Yes`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAll::Yes`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAll::Yes`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAll::Yes`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAll::No`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAll::No`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAll::No`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAll::No`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumUnitToJsonValue {
            ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentYes`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAllResult::ConfidentYes`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentYes`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAllResult::ConfidentYes`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentNo`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAllResult::ConfidentNo`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentNo`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAllResult::ConfidentNo`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAllResult::Unconfident`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAllResult::Unconfident`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`malamute::OneVsAllResult::Unconfident`),
            instantiation: LinInstantiation {
                path: ItemPath(`malamute::OneVsAllResult::Unconfident`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::AssocRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<malamute::OneVsAll as core::default::Default(0)>::default`,
                    TraitItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`<malamute::OneVsAll as core::default::Default(0)>::default`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::AssocRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                    TraitItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::UnveilAssocRitchie {
            path: TraitForTypeItemPath(
                `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                TraitItemKind::AssocRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
            instantiation: LinInstantiation {
                path: ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Continue`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Continue`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Continue`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Continue`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field_ty_leash_class: Copyable,
            field: Tuple {
                index: 0,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(`core::ops::ControlFlow::Break`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Break`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(`core::ops::ControlFlow::Break`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Break`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                template_arguments: [
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                            },
                        ),
                    ),
                    LinTemplateArgument::Type(
                        LinType::PathLeading(
                            LinTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                    ),
                ],
            },
            path: TypeVariantPath(`core::ops::ControlFlow::Break`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Break`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`core::ops::ControlFlow::Break`),
            instantiation: LinInstantiation {
                path: ItemPath(`core::ops::ControlFlow::Break`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                    (
                        HirTemplateVariable::Type(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        LinTermVariableResolution::Explicit(
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::basic::unit`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field_ty_leash_class: Other,
            field: Tuple {
                index: 0,
            },
        },
    },
]
```