```rust
[
    (
        Linket {
            data: LinketData::EnumUnitToJsonValue {
                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
            },
        },
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumUnitToJsonValue {
                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
            },
        },
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        Some(
            VmirRegion {
                linket: Linket {
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
        Some(
            VmirRegion {
                linket: Linket {
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
                        VmirExprData::Linket {
                            linket_impl: VirtualLinketImpl(
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
                        VmirExprData::Linket {
                            linket_impl: VirtualLinketImpl(
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
                        VmirExprData::Linket {
                            linket_impl: VirtualLinketImpl(
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
        Some(
            VmirRegion {
                linket: Linket {
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
                        VmirExprData::Linket {
                            linket_impl: VirtualLinketImpl(
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
                        VmirExprData::Linket {
                            linket_impl: VirtualLinketImpl(
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
                        VmirExprData::Linket {
                            linket_impl: VirtualLinketImpl(
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
                field_ty_leash_class: Copyable,
                field: Tuple {
                    index: 0,
                },
            },
        },
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
                field_ty_leash_class: Other,
                field: Tuple {
                    index: 0,
                },
            },
        },
        None,
    ),
]
```