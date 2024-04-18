```rust
[
    (
        Linkage {
            data: LinkageData::EnumU8ToJsonValue {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
            },
        },
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
        Linkage {
            data: LinkageData::EnumU8ToJsonValue {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
            },
        },
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        Some(
            VmirRegion {
                linkage: Linkage {
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
                root_expr: VmirExprIdx(
                    1,
                ),
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::PrincipalEntityPath,
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    0..1,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                0,
                            ),
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                    ],
                },
            },
        ),
    ),
    (
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
        Some(
            VmirRegion {
                linkage: Linkage {
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
                root_expr: VmirExprIdx(
                    7,
                ),
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Variable {
                            place_idx: PlaceIdx(0),
                            qual: Ref,
                        },
                        VmirExprData::PrincipalEntityPath,
                        VmirExprData::Linkage {
                            linkage_impl: Linkage {
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
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        1,
                                    ),
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::PrincipalEntityPath,
                        VmirExprData::Linkage {
                            linkage_impl: Linkage {
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
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        3,
                                    ),
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Literal {
                            value: Unit(
                                (),
                            ),
                        },
                        VmirExprData::Linkage {
                            linkage_impl: Linkage {
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
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        5,
                                    ),
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    3..4,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                2,
                            ),
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                4,
                            ),
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                6,
                            ),
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Match {
                            opd: VmirExprIdx(
                                0,
                            ),
                            case_branches: [
                                VmirCaseBranch {
                                    pattern: VmirPattern {
                                        restructive_pattern: VmirRestructivePattern::UnitPath,
                                        destructive_pattern: None,
                                    },
                                    stmts: VmirStmtIdxRange(
                                        ArenaIdxRange(
                                            0..1,
                                        ),
                                    ),
                                },
                                VmirCaseBranch {
                                    pattern: VmirPattern {
                                        restructive_pattern: VmirRestructivePattern::UnitPath,
                                        destructive_pattern: None,
                                    },
                                    stmts: VmirStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                                VmirCaseBranch {
                                    pattern: VmirPattern {
                                        restructive_pattern: VmirRestructivePattern::UnitPath,
                                        destructive_pattern: None,
                                    },
                                    stmts: VmirStmtIdxRange(
                                        ArenaIdxRange(
                                            2..3,
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                },
            },
        ),
    ),
    (
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
        Some(
            VmirRegion {
                linkage: Linkage {
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
                root_expr: VmirExprIdx(
                    7,
                ),
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Variable {
                            place_idx: PlaceIdx(0),
                            qual: Ref,
                        },
                        VmirExprData::PrincipalEntityPath,
                        VmirExprData::Linkage {
                            linkage_impl: Linkage {
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
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        1,
                                    ),
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::PrincipalEntityPath,
                        VmirExprData::Linkage {
                            linkage_impl: Linkage {
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
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        3,
                                    ),
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Literal {
                            value: Unit(
                                (),
                            ),
                        },
                        VmirExprData::Linkage {
                            linkage_impl: Linkage {
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
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        5,
                                    ),
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    3..4,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                2,
                            ),
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                4,
                            ),
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                6,
                            ),
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Match {
                            opd: VmirExprIdx(
                                0,
                            ),
                            case_branches: [
                                VmirCaseBranch {
                                    pattern: VmirPattern {
                                        restructive_pattern: VmirRestructivePattern::UnitPath,
                                        destructive_pattern: None,
                                    },
                                    stmts: VmirStmtIdxRange(
                                        ArenaIdxRange(
                                            0..1,
                                        ),
                                    ),
                                },
                                VmirCaseBranch {
                                    pattern: VmirPattern {
                                        restructive_pattern: VmirRestructivePattern::UnitPath,
                                        destructive_pattern: None,
                                    },
                                    stmts: VmirStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                                VmirCaseBranch {
                                    pattern: VmirPattern {
                                        restructive_pattern: VmirRestructivePattern::UnitPath,
                                        destructive_pattern: None,
                                    },
                                    stmts: VmirStmtIdxRange(
                                        ArenaIdxRange(
                                            2..3,
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                },
            },
        ),
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
]
```