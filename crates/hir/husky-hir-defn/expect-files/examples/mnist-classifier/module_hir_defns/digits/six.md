[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                        FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                6,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 268,
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
                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                        FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                7,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 270,
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
                    path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                121,
                            ),
                            Lazy(
                                HirLazyExprRegion(
                                    Id {
                                        value: 2,
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
                    path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                                        FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                                            FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                    path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                                        FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                            35,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                                                            value: 10,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 5,
                                            },
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 4,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 6,
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
                                                owner_hir_expr_idx: 8,
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
                                                owner_hir_expr_idx: 10,
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
                                            data: Binary {
                                                lopd: 9,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 11,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 12,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 165,
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
                                            data: Literal(
                                                F32(
                                                    TermF32Literal(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 13,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 14,
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
                                            data: MemoizedField {
                                                owner_hir_expr_idx: 16,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 291,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 454,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 17,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 288,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 464,
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
                                            data: Literal(
                                                F32(
                                                    TermF32Literal(
                                                        Id {
                                                            value: 17,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 18,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 19,
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
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 21,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 211,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MemoizedField {
                                                owner_hir_expr_idx: 22,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 282,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 480,
                                                            },
                                                        ),
                                                    ),
                                                ),
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
                                                self_argument: 24,
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
                                            data: MethodFnCall {
                                                self_argument: 23,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 361,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 483,
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
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 44,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        25,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 27,
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
                                                            value: 3,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 28,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 29,
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
                                                self_argument: 31,
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
                                                owner_hir_expr_idx: 32,
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
                                                opd_hir_expr_idx: 33,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    2..8,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 15,
                                                conversion: None,
                                            },
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 7,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 20,
                                                conversion: None,
                                            },
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 26,
                                        },
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 30,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr_idx: 34,
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
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 476,
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
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `relative_end`,
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