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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [],
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
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
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
            field: Tuple {
                index: 0,
            },
        },
    },
]
```