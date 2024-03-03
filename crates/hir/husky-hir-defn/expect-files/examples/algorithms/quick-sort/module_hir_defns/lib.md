[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`quick_sort::quick_sort`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort`, `Ritchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateSvarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: HirTemplateParameterData::Type {
                                        ident: `T`,
                                        traits: [
                                            HirTrait {
                                                trai_path: TraitPath(`core::cmp::Ord`),
                                                template_arguments: [],
                                            },
                                        ],
                                    },
                                },
                            ],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_expr_idx: 1,
                                    contract: BorrowMut,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::Svar(
                                                        HirTypeSvar::Type {
                                                            attrs: HirTemplateSvarAttrs {
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
                                        FugitivePath(`quick_sort::quick_sort`, `Ritchie(
                                            Fn,
                                        )`),
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
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: Some(
                                            RefMut,
                                        ),
                                        ident: `arr`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `T`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateVar::Type(
                                                HirTypeSvar::Type {
                                                    attrs: HirTemplateSvarAttrs {
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `arr`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            10,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`quick_sort::quick_sort`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `len`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::Slice(0)::len`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::Svar(
                                                                        HirTypeSvar::Type {
                                                                            attrs: HirTemplateSvarAttrs {
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
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::ISize(
                                                    TermISizeLiteral {
                                                        value: 0,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::USize(
                                                    TermUSizeLiteral {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 5,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 6,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 7,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::isize`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                    Fn,
                                                )`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::Svar(
                                                                        HirTypeSvar::Type {
                                                                            attrs: HirTemplateSvarAttrs {
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
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: BorrowMut,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 2,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        3,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: RefMut {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                    lifetime: None,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 3,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 3,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        8,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..3,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
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
                                            contract: Pure,
                                            initial_value: 2,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 9,
                                            coersion: None,
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `len`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerComptimeSvarEntry {
                                                name: HirEagerComptimeSvarName::Ident(
                                                    `T`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateVar::Type(
                                                    HirTypeSvar::Type {
                                                        attrs: HirTemplateSvarAttrs {
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `arr`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `len`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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
                    path: FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateSvarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: HirTemplateParameterData::Type {
                                        ident: `T`,
                                        traits: [
                                            HirTrait {
                                                trai_path: TraitPath(`core::cmp::Ord`),
                                                template_arguments: [],
                                            },
                                        ],
                                    },
                                },
                            ],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_expr_idx: 1,
                                    contract: BorrowMut,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::Svar(
                                                        HirTypeSvar::Type {
                                                            attrs: HirTemplateSvarAttrs {
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
                                HirEagerParenateParameter::Simple {
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
                                HirEagerParenateParameter::Simple {
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
                                        FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                            Fn,
                                        )`),
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
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: Some(
                                            RefMut,
                                        ),
                                        ident: `arr`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `low`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `high`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `T`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateVar::Type(
                                                HirTypeSvar::Type {
                                                    attrs: HirTemplateSvarAttrs {
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `arr`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `low`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `high`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            20,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 1,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 2,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`quick_sort::partition`, `Ritchie(
                                                    Fn,
                                                )`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::Svar(
                                                                        HirTypeSvar::Type {
                                                                            attrs: HirTemplateSvarAttrs {
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
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: BorrowMut,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 2,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: RefMut {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                    lifetime: None,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 3,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 3,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        6,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::ISize(
                                                    TermISizeLiteral {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 10,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 11,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                    Fn,
                                                )`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::Svar(
                                                                        HirTypeSvar::Type {
                                                                            attrs: HirTemplateSvarAttrs {
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
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: BorrowMut,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 2,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        8,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: RefMut {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                    lifetime: None,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 3,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        9,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 3,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        12,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::ISize(
                                                    TermISizeLiteral {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 15,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 16,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                    Fn,
                                                )`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::Svar(
                                                                        HirTypeSvar::Type {
                                                                            attrs: HirTemplateSvarAttrs {
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
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: BorrowMut,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 2,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        14,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: RefMut {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                    lifetime: None,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 3,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        17,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 3,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        18,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    4..5,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
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
                                            contract: Pure,
                                            initial_value: 7,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 13,
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
                                            expr_idx: 19,
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
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `p`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerComptimeSvarEntry {
                                                name: HirEagerComptimeSvarName::Ident(
                                                    `T`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateVar::Type(
                                                    HirTypeSvar::Type {
                                                        attrs: HirTemplateSvarAttrs {
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `arr`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `low`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `high`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `p`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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
                    path: FugitivePath(`quick_sort::partition`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`quick_sort::partition`, `Ritchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateSvarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: HirTemplateParameterData::Type {
                                        ident: `T`,
                                        traits: [
                                            HirTrait {
                                                trai_path: TraitPath(`core::cmp::Ord`),
                                                template_arguments: [],
                                            },
                                        ],
                                    },
                                },
                            ],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_expr_idx: 1,
                                    contract: BorrowMut,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::Svar(
                                                        HirTypeSvar::Type {
                                                            attrs: HirTemplateSvarAttrs {
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
                                HirEagerParenateParameter::Simple {
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
                                HirEagerParenateParameter::Simple {
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
                                        FugitivePath(`quick_sort::partition`, `Ritchie(
                                            Fn,
                                        )`),
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
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: Some(
                                            RefMut,
                                        ),
                                        ident: `arr`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `low`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `high`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `T`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateVar::Type(
                                                HirTypeSvar::Type {
                                                    attrs: HirTemplateSvarAttrs {
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `arr`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `low`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `high`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            56,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`quick_sort::partition`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 1,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::ISize(
                                                    TermISizeLiteral {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 3,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::Bool(
                                                    true,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::ISize(
                                                    TermISizeLiteral {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 8,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 12,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 11,
                                                items: [
                                                    13,
                                                ],
                                            },
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 15,
                                                items: [
                                                    16,
                                                ],
                                            },
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 14,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 17,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::ISize(
                                                    TermISizeLiteral {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 19,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 20,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::ISize(
                                                    TermISizeLiteral {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 22,
                                                opr: AssignClosed(
                                                    Sub,
                                                ),
                                                ropd: 23,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::ISize(
                                                    TermISizeLiteral {
                                                        value: 0,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 25,
                                                opr: Comparison(
                                                    Geq,
                                                ),
                                                ropd: 26,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 29,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 28,
                                                items: [
                                                    30,
                                                ],
                                            },
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 32,
                                                items: [
                                                    33,
                                                ],
                                            },
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 31,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 34,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 27,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 35,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::ISize(
                                                    TermISizeLiteral {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 37,
                                                opr: AssignClosed(
                                                    Sub,
                                                ),
                                                ropd: 38,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 40,
                                                opr: Comparison(
                                                    Geq,
                                                ),
                                                ropd: 41,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 44,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 46,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 43,
                                                self_contract: BorrowMut,
                                                ident: `swap`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::Slice(0)::swap`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::Svar(
                                                                        HirTypeSvar::Type {
                                                                            attrs: HirTemplateSvarAttrs {
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
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::SelfLifetime,
                                                            ),
                                                            HirTermSvarResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        45,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        47,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                lifetime: None,
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 50,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 52,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 49,
                                                self_contract: BorrowMut,
                                                ident: `swap`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::Slice(0)::swap`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::Svar(
                                                                        HirTypeSvar::Type {
                                                                            attrs: HirTemplateSvarAttrs {
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
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::SelfLifetime,
                                                            ),
                                                            HirTermSvarResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        51,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        53,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    10..16,
                                                ),
                                            },
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 21,
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
                                            expr_idx: 39,
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
                                            expr_idx: 48,
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
                                            expr_idx: 10,
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
                                                hir_eager_expr_idx: 18,
                                                conversion: None,
                                            },
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 24,
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
                                                hir_eager_expr_idx: 36,
                                                conversion: None,
                                            },
                                            stmts: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 42,
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
                                            contract: Pure,
                                            initial_value: 2,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 5,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 6,
                                            coersion: None,
                                        },
                                        While {
                                            condition: Other {
                                                hir_eager_expr_idx: 7,
                                                conversion: None,
                                            },
                                            stmts: ArenaIdxRange(
                                                5..10,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 54,
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
                                            expr_idx: 55,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: MutableStackOwned {
                                                            place: Idx(
                                                                PlaceIdx(
                                                                    ShiftedU32(
                                                                        2,
                                                                    ),
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
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `pivot`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `store_index`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `last_index`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerComptimeSvarEntry {
                                                name: HirEagerComptimeSvarName::Ident(
                                                    `T`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateVar::Type(
                                                    HirTypeSvar::Type {
                                                        attrs: HirTemplateSvarAttrs {
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `arr`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `low`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `high`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `pivot`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `store_index`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `last_index`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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
                    path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                            Fn,
                        )`),
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
                                        FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                            Fn,
                                        )`),
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
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
                                            FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    4,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    65,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    2,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    31,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 4,
                                            },
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    99,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    2,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    83,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    782,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::NewList {
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
                                                element_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
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
                                                                value: 6,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            contract: Move,
                                            initial_value: 12,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `v`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `v`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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
                    path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                            Fn,
                        )`),
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
                                        FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                            Fn,
                                        )`),
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
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
                                            FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::String(
                                                    StringLiteralTokenData {
                                                        data: "beach",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::String(
                                                    StringLiteralTokenData {
                                                        data: "hotel",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::String(
                                                    StringLiteralTokenData {
                                                        data: "airplane",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::String(
                                                    StringLiteralTokenData {
                                                        data: "car",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::String(
                                                    StringLiteralTokenData {
                                                        data: "house",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::String(
                                                    StringLiteralTokenData {
                                                        data: "art",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::NewList {
                                                items: [
                                                    1,
                                                    2,
                                                    3,
                                                    4,
                                                    5,
                                                    6,
                                                ],
                                                element_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Ref`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Constant(
                                                                StaticLifetime,
                                                            ),
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`core::str::str`, `Extern`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
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
                                            contract: Move,
                                            initial_value: 7,
                                            coersion: None,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `strs`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `strs`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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