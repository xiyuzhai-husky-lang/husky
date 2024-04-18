```rust
[
    Linkage {
        data: LinkageData::EnumU8ToJsonValue {
            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 5,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 5,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 6,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 6,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumU8ToJsonValue {
            ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 8,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 8,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 9,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 9,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 10,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 10,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::AssocRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<malamute::OneVsAll as core::default::Default(0)>::default`,
                    TraitItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::AssocRitchie {
            path: AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                    TraitItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::UnveilAssocFn {
            path: TraitForTypeItemPath(
                `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                TraitItemKind::AssocRitchie(
                    RitchieItemKind::Fn,
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::EnumVariantConstructor {
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
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 100,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Comptime,
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
                                    class: Comptime,
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
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
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
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 100,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Comptime,
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
                                    class: Comptime,
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
    Linkage {
        data: LinkageData::EnumVariantDestructor {
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
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 100,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Comptime,
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
                                    class: Comptime,
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
    Linkage {
        data: LinkageData::EnumVariantField {
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 100,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Comptime,
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
                                    class: Comptime,
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
    Linkage {
        data: LinkageData::EnumVariantConstructor {
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
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 101,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Comptime,
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
                                    class: Comptime,
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
    Linkage {
        data: LinkageData::EnumVariantDiscriminator {
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
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 101,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Comptime,
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
                                    class: Comptime,
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
    Linkage {
        data: LinkageData::EnumVariantDestructor {
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
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 101,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Comptime,
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
                                    class: Comptime,
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
    Linkage {
        data: LinkageData::EnumVariantField {
            path: TypeVariantPath(
                ItemPathId(
                    Id {
                        value: 101,
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Comptime,
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
                                    class: Comptime,
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