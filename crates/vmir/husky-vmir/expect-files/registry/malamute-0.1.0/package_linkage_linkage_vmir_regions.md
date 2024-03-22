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
                root_expr: 2,
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::PrincipalEntityPath,
                        VmirExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                            destroyers: ArenaIdxRange(
                                1..1,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Eval {
                            expr: 1,
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
                root_expr: 8,
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                    expr: 2,
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                    expr: 4,
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Literal,
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                    expr: 6,
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Block {
                            stmts: ArenaIdxRange(
                                4..5,
                            ),
                            destroyers: ArenaIdxRange(
                                1..1,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Eval {
                            expr: 3,
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: 5,
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: 7,
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Match {
                            opd: 1,
                            case_branches: [
                                VmirCaseBranch {
                                    pattern: VmirPatternIdx {
                                        restructive_pattern: 1,
                                        destructive_pattern: None,
                                    },
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                                VmirCaseBranch {
                                    pattern: VmirPatternIdx {
                                        restructive_pattern: 2,
                                        destructive_pattern: None,
                                    },
                                    stmts: ArenaIdxRange(
                                        2..3,
                                    ),
                                },
                                VmirCaseBranch {
                                    pattern: VmirPatternIdx {
                                        restructive_pattern: 3,
                                        destructive_pattern: None,
                                    },
                                    stmts: ArenaIdxRange(
                                        3..4,
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
                root_expr: 8,
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                    expr: 2,
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                    expr: 4,
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Literal,
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                                        attrs: HirTemplateSvarAttrs {
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
                                    expr: 6,
                                    coersion: VmirCoersion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Block {
                            stmts: ArenaIdxRange(
                                4..5,
                            ),
                            destroyers: ArenaIdxRange(
                                1..1,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Eval {
                            expr: 3,
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: 5,
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: 7,
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Match {
                            opd: 1,
                            case_branches: [
                                VmirCaseBranch {
                                    pattern: VmirPatternIdx {
                                        restructive_pattern: 1,
                                        destructive_pattern: None,
                                    },
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                                VmirCaseBranch {
                                    pattern: VmirPatternIdx {
                                        restructive_pattern: 2,
                                        destructive_pattern: None,
                                    },
                                    stmts: ArenaIdxRange(
                                        2..3,
                                    ),
                                },
                                VmirCaseBranch {
                                    pattern: VmirPatternIdx {
                                        restructive_pattern: 3,
                                        destructive_pattern: None,
                                    },
                                    stmts: ArenaIdxRange(
                                        3..4,
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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
                                    attrs: HirTemplateSvarAttrs {
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