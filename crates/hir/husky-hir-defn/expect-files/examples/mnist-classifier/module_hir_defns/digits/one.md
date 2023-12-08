[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                8,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 247,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Lazy(
                                246,
                            ),
                            Lazy(
                                HirLazyExprRegion(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 277,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            9,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 458,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                F32(
                                                    TermF32Literal(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 4,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 5,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 7,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..4,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 6,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr_idx: 8,
                                            coersion: Some(
                                                WrapInSome,
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 380,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `cc`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 277,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            11,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 458,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                F32(
                                                    TermF32Literal(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 4,
                                                opr: Comparison(
                                                    Leq,
                                                ),
                                                ropd: 5,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 7,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 143,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 457,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 9,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..4,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 6,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr_idx: 10,
                                            coersion: Some(
                                                WrapInSome,
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 380,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `cc`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 277,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            17,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 458,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                F32(
                                                    TermF32Literal(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 4,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 5,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 7,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 266,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                F32(
                                                    TermF32Literal(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 8,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 11,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 12,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 14,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 266,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 13,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 15,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..5,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 6,
                                                conversion: None,
                                            },
                                        },
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 10,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr_idx: 16,
                                            coersion: Some(
                                                WrapInSome,
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 380,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `cc`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]