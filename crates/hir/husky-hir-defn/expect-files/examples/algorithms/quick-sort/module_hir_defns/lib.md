[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                        traits: [
                                            HirTrait(
                                                Id {
                                                    value: 19,
                                                },
                                            ),
                                        ],
                                    },
                                },
                            ],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: BorrowMut,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::Symbol(
                                                        Type {
                                                            attrs: HirTemplateSymbolAttrs {
                                                                class: Comptime,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: false,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
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
                                        symbol_modifier: Some(
                                            RefMut,
                                        ),
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 184,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `T`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateSymbol::Type(
                                                Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                    ],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `arr`,
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
                            12,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
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
                                                            value: 148,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 226,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    Symbol(
                                                                        Type {
                                                                            attrs: HirTemplateSymbolAttrs {
                                                                                class: Comptime,
                                                                            },
                                                                            variance: None,
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Fugitive(
                                                        FugitivePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 253,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                ISize(
                                                    TermISizeLiteral(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
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
                                            data: Literal(
                                                USize(
                                                    TermUSizeLiteral(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 6,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 7,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 46,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 8,
                                                opr: As,
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: FunctionFnCall {
                                                path: FugitivePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 253,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 3,
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: BorrowMut,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 20,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: RefMut {
                                                                    guard: Left(
                                                                        StackLocationIdx(
                                                                            ShiftedU32(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        10,
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
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..3,
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
                                        Eval {
                                            expr_idx: 11,
                                            coersion: None,
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
                                                        value: 148,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerComptimeSymbolEntry {
                                                name: HirEagerComptimeSymbolName::Ident(
                                                    `T`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateSymbol::Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `arr`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `len`,
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
                    path: FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                        traits: [
                                            HirTrait(
                                                Id {
                                                    value: 19,
                                                },
                                            ),
                                        ],
                                    },
                                },
                            ],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: BorrowMut,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::Symbol(
                                                        Type {
                                                            attrs: HirTemplateSymbolAttrs {
                                                                class: Comptime,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: false,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::isize`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 3,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::isize`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
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
                                        symbol_modifier: Some(
                                            RefMut,
                                        ),
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 184,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 186,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 187,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `T`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateSymbol::Type(
                                                Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                    ],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `arr`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `low`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `high`,
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
                            23,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 1,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 2,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Fugitive(
                                                        FugitivePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 254,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: FunctionFnCall {
                                                path: FugitivePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 254,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 4,
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: BorrowMut,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 20,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: RefMut {
                                                                    guard: Left(
                                                                        StackLocationIdx(
                                                                            ShiftedU32(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        6,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        7,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Fugitive(
                                                        FugitivePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 253,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
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
                                            data: Literal(
                                                ISize(
                                                    TermISizeLiteral(
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
                                                lopd: 12,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 13,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: FunctionFnCall {
                                                path: FugitivePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 253,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 9,
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: BorrowMut,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 20,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        10,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: RefMut {
                                                                    guard: Left(
                                                                        StackLocationIdx(
                                                                            ShiftedU32(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        11,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        14,
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
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Fugitive(
                                                        FugitivePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 253,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
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
                                            data: Literal(
                                                ISize(
                                                    TermISizeLiteral(
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
                                                lopd: 18,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 19,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: FunctionFnCall {
                                                path: FugitivePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 253,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 16,
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: BorrowMut,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 20,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        17,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: RefMut {
                                                                    guard: Left(
                                                                        StackLocationIdx(
                                                                            ShiftedU32(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        20,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        21,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    4..5,
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
                                            initial_value: 8,
                                        },
                                        Eval {
                                            expr_idx: 15,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 22,
                                            coersion: None,
                                            discarded: false,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 3,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    1..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
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
                                                        value: 188,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerComptimeSymbolEntry {
                                                name: HirEagerComptimeSymbolName::Ident(
                                                    `T`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateSymbol::Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `arr`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `low`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `high`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `p`,
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
                    path: FugitivePath(`quick_sort::partition`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::partition`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                        traits: [
                                            HirTrait(
                                                Id {
                                                    value: 19,
                                                },
                                            ),
                                        ],
                                    },
                                },
                            ],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: BorrowMut,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::Symbol(
                                                        Type {
                                                            attrs: HirTemplateSymbolAttrs {
                                                                class: Comptime,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: false,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::isize`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 3,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::isize`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::isize`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`quick_sort::partition`, `FunctionFn`),
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
                                        symbol_modifier: Some(
                                            RefMut,
                                        ),
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 184,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 186,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 187,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `T`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateSymbol::Type(
                                                Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                    ],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `arr`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `low`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `high`,
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
                            63,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`quick_sort::partition`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 1,
                                                opr: As,
                                                ropd: 2,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                ISize(
                                                    TermISizeLiteral(
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
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 5,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                Bool(
                                                    true,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                ISize(
                                                    TermISizeLiteral(
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
                                                lopd: 9,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 10,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 13,
                                                opr: As,
                                                ropd: 14,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Index {
                                                owner_hir_expr_idx: 12,
                                                items: [
                                                    15,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
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
                                            data: Index {
                                                owner_hir_expr_idx: 17,
                                                items: [
                                                    18,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 16,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 19,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                ISize(
                                                    TermISizeLiteral(
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
                                                lopd: 21,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 22,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                ISize(
                                                    TermISizeLiteral(
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
                                                lopd: 24,
                                                opr: AssignClosed(
                                                    Sub,
                                                ),
                                                ropd: 25,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                ISize(
                                                    TermISizeLiteral(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 27,
                                                opr: Comparison(
                                                    Geq,
                                                ),
                                                ropd: 28,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 31,
                                                opr: As,
                                                ropd: 32,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Index {
                                                owner_hir_expr_idx: 30,
                                                items: [
                                                    33,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
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
                                            data: Index {
                                                owner_hir_expr_idx: 35,
                                                items: [
                                                    36,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 34,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 37,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 29,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 38,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                ISize(
                                                    TermISizeLiteral(
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
                                                lopd: 40,
                                                opr: AssignClosed(
                                                    Sub,
                                                ),
                                                ropd: 41,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 43,
                                                opr: Comparison(
                                                    Geq,
                                                ),
                                                ropd: 44,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 47,
                                                opr: As,
                                                ropd: 48,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 50,
                                                opr: As,
                                                ropd: 51,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 46,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 149,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 227,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    Symbol(
                                                                        Type {
                                                                            attrs: HirTemplateSymbolAttrs {
                                                                                class: Comptime,
                                                                            },
                                                                            variance: None,
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        (
                                                            Type(
                                                                SelfLifetime,
                                                            ),
                                                            SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 2,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        49,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 2,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        52,
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
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                guard: Left(
                                                    StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 55,
                                                opr: As,
                                                ropd: 56,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
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
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 58,
                                                opr: As,
                                                ropd: 59,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 54,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 149,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 227,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    Symbol(
                                                                        Type {
                                                                            attrs: HirTemplateSymbolAttrs {
                                                                                class: Comptime,
                                                                            },
                                                                            variance: None,
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        (
                                                            Type(
                                                                SelfLifetime,
                                                            ),
                                                            SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 2,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        57,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 2,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        60,
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
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    10..16,
                                                ),
                                            },
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 23,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 42,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Break,
                                        Eval {
                                            expr_idx: 53,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        While {
                                            condition: Other {
                                                hir_eager_expr_idx: 20,
                                                conversion: None,
                                            },
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 26,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        While {
                                            condition: Other {
                                                hir_eager_expr_idx: 39,
                                                conversion: None,
                                            },
                                            stmts: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 45,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 3,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 6,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 7,
                                        },
                                        While {
                                            condition: Other {
                                                hir_eager_expr_idx: 8,
                                                conversion: None,
                                            },
                                            stmts: ArenaIdxRange(
                                                5..10,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 61,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 62,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: MutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ),
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
                                                        value: 190,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 191,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 192,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerComptimeSymbolEntry {
                                                name: HirEagerComptimeSymbolName::Ident(
                                                    `T`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateSymbol::Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `arr`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `low`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `high`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `pivot`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `store_index`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `last_index`,
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
                    path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            13,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    4,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    65,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    2,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    31,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 4,
                                            },
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    99,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    2,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    83,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    782,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: NewList {
                                                items: [
                                                    1,
                                                    2,
                                                    3,
                                                    5,
                                                    6,
                                                    7,
                                                    8,
                                                    9,
                                                    10,
                                                    11,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
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
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 31,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            initial_value: 12,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 194,
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
                                                    `v`,
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
                    path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Literal(
                                                String(
                                                    StringLiteralData(
                                                        Id {
                                                            value: 18,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                String(
                                                    StringLiteralData(
                                                        Id {
                                                            value: 19,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                String(
                                                    StringLiteralData(
                                                        Id {
                                                            value: 20,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                String(
                                                    StringLiteralData(
                                                        Id {
                                                            value: 21,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                String(
                                                    StringLiteralData(
                                                        Id {
                                                            value: 22,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                String(
                                                    StringLiteralData(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: NewList {
                                                items: [
                                                    1,
                                                    2,
                                                    3,
                                                    4,
                                                    5,
                                                    6,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
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
                                            initial_value: 7,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 196,
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
                                                    `strs`,
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