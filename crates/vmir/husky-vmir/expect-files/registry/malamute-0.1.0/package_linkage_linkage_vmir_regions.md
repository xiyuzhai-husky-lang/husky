```rust
[
    (
        Linkage {
            data: LinkageData::EnumUnitToJsonValue {
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
                path: TypeVariantPath(`malamute::OneVsAll::Yes`),
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
                path: TypeVariantPath(`malamute::OneVsAll::Yes`),
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
                path: TypeVariantPath(`malamute::OneVsAll::No`),
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
                path: TypeVariantPath(`malamute::OneVsAll::No`),
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
            data: LinkageData::EnumUnitToJsonValue {
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
                path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentYes`),
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
                path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentYes`),
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
                path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentNo`),
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
                path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentNo`),
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
                path: TypeVariantPath(`malamute::OneVsAllResult::Unconfident`),
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
                path: TypeVariantPath(`malamute::OneVsAllResult::Unconfident`),
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
                            coercion: Some(
                                VmirCoercion::Trivial,
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
                            linkage_impl: VirtualLinkageImpl(
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
                                        path: TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        instantiation: LinInstantiation {
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
                            ),
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        1,
                                    ),
                                    coercion: VmirCoercion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::PrincipalEntityPath,
                        VmirExprData::Linkage {
                            linkage_impl: VirtualLinkageImpl(
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
                                        path: TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        instantiation: LinInstantiation {
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
                            ),
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        3,
                                    ),
                                    coercion: VmirCoercion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Literal {
                            value: Unit(
                                (),
                            ),
                        },
                        VmirExprData::Linkage {
                            linkage_impl: VirtualLinkageImpl(
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
                                        path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
                                        instantiation: LinInstantiation {
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
                            ),
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        5,
                                    ),
                                    coercion: VmirCoercion::Trivial,
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
                            coercion: Some(
                                VmirCoercion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                4,
                            ),
                            coercion: Some(
                                VmirCoercion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                6,
                            ),
                            coercion: Some(
                                VmirCoercion::Trivial,
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
            data: LinkageData::UnveilAssocRitchie {
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
                    data: LinkageData::UnveilAssocRitchie {
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
                            linkage_impl: VirtualLinkageImpl(
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
                                        path: TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        instantiation: LinInstantiation {
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
                            ),
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        1,
                                    ),
                                    coercion: VmirCoercion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::PrincipalEntityPath,
                        VmirExprData::Linkage {
                            linkage_impl: VirtualLinkageImpl(
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
                                        path: TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        instantiation: LinInstantiation {
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
                            ),
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        3,
                                    ),
                                    coercion: VmirCoercion::Trivial,
                                },
                            ],
                        },
                        VmirExprData::Literal {
                            value: Unit(
                                (),
                            ),
                        },
                        VmirExprData::Linkage {
                            linkage_impl: VirtualLinkageImpl(
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
                                        path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
                                        instantiation: LinInstantiation {
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
                            ),
                            arguments: [
                                VmirArgument::Simple {
                                    expr: VmirExprIdx(
                                        5,
                                    ),
                                    coercion: VmirCoercion::Trivial,
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
                            coercion: Some(
                                VmirCoercion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                4,
                            ),
                            coercion: Some(
                                VmirCoercion::Trivial,
                            ),
                            discarded: false,
                        },
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                6,
                            ),
                            coercion: Some(
                                VmirCoercion::Trivial,
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
                path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
                instantiation: LinInstantiation {
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
                path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
                instantiation: LinInstantiation {
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
                path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
                instantiation: LinInstantiation {
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
        None,
    ),
    (
        Linkage {
            data: LinkageData::EnumVariantField {
                path: TypeVariantPath(`core::ops::ControlFlow::Continue`),
                instantiation: LinInstantiation {
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
                path: TypeVariantPath(`core::ops::ControlFlow::Break`),
                instantiation: LinInstantiation {
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
                path: TypeVariantPath(`core::ops::ControlFlow::Break`),
                instantiation: LinInstantiation {
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
                path: TypeVariantPath(`core::ops::ControlFlow::Break`),
                instantiation: LinInstantiation {
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
        None,
    ),
    (
        Linkage {
            data: LinkageData::EnumVariantField {
                path: TypeVariantPath(`core::ops::ControlFlow::Break`),
                instantiation: LinInstantiation {
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
        None,
    ),
]
```