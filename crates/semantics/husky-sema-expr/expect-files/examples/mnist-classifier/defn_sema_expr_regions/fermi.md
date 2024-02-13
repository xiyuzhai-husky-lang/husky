[
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                            Fn,
                        )`),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `concave_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `concave_components`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    self_contract: Leash,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `collect_leashes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: Leash,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vec Leash ConcaveComponent`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSvar(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`ConcaveComponent`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::VecFunctor {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`covariant Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`covariant Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Prefix {
                                    opr: LeashType,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Prefix {
                                    opr: Option,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        6,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::NewList {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    items: [],
                                    element_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `templates`,
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `templates`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            27,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: StackPure {
                                                            location: PlaceIdx(
                                                                ShiftedU32(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSvar(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        10,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `templates`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `templates`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        10,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                13,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `matches`,
                                    regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `others`,
                                    regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `template`,
                                    regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        16,
                                    ),
                                    self_contract: BorrowMut,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `pop_with_largest_opt_f32`,
                                        regional_token_idx: RegionalTokenIdx(
                                            44,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: BorrowMut,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec Leash ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Regular(
                                                        FlyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FlyTerm {
                                                                place: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Option Leash ConcaveComponent`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: MutableStackOwned {
                                                            location: PlaceIdx(
                                                                ShiftedU32(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSvar(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`Leash ConcaveComponent`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        (
                                                            EthSvar(`'self_lifetime`),
                                                            FlyTermSymbolResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FlyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    17,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            4,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        47,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        15,
                                    ),
                                    self_contract: BorrowMut,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `push`,
                                        regional_token_idx: RegionalTokenIdx(
                                            40,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::push`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: BorrowMut,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Regular(
                                                        FlyRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: FlyTerm {
                                                                place: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`Option Leash ConcaveComponent`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`unit`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::vec::Vec(0)::push`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: MutableStackOwned {
                                                            location: PlaceIdx(
                                                                ShiftedU32(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSvar(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`Option Leash ConcaveComponent`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        (
                                                            EthSvar(`'self_lifetime`),
                                                            FlyTermSymbolResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FlyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Option Leash ConcaveComponent`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    18,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    19,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`fn((move  Vec Option Leash ConcaveComponent, move  Vec Leash ConcaveComponent) -> FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    20,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn((move  Vec Option Leash ConcaveComponent, move  Vec Leash ConcaveComponent) -> FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `matches`,
                                    regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    21,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `others`,
                                    regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    22,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        20,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FlyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    21,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        53,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FlyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vec Leash ConcaveComponent`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    22,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    23,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            3..7,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`never`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    24,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`never`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 3,
                                        },
                                        variables: ArenaIdxRange(
                                            4..5,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            33,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        14,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        19,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFlyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon_token: Some(
                                            ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                            ),
                                        ),
                                        ty_sema_expr_idx: Some(
                                            SemaExprIdx(
                                                7,
                                            ),
                                        ),
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            19,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFlyCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    particulars: SemaForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: SemaExprIdx(
                                            11,
                                        ),
                                        range: SemaForBetweenRange {
                                            initial_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: None,
                                                kind: LowerClosed,
                                            },
                                            final_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        10,
                                                    ),
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    for_loop_var_symbol_idx: 3,
                                    eol_colon: EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                30,
                                            ),
                                        },
                                    ),
                                    block: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..3,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            49,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        23,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFlyCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`never`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    25,
                    (
                        SemaExprIdx(
                            24,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 59,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: Eth(
                                Ritchie(
                                    EthRitchie(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        Application(
                                            EthApplication(
                                                Id {
                                                    value: 60,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        Application(
                                            EthApplication(
                                                Id {
                                                    value: 59,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Eth(
                                        Ritchie(
                                            EthRitchie(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        3,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        4,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`ConcaveComponent`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        5,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        6,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Option Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        7,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec Option Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Leash Vec ConcaveComponent`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                MutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec Leash ConcaveComponent`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                MutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec Option Leash ConcaveComponent`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`i32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    solid_terms: SolTerms {
                        entries: [],
                    },
                    hollow_terms: HolTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::Sort,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`covariant Type -> Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Type`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::ExplicitCurry {
                                                        variance: Covariant,
                                                        parameter_hvar: None,
                                                        parameter_ty: FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Eth(
                                                                EthTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Eth(
                                                                EthTerm(`Type`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Type`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Type`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Type`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: Universe(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                Universe(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Vec Option Leash ConcaveComponent`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        3,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        4,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Option Leash ConcaveComponent`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn((move  Vec Option Leash ConcaveComponent, move  Vec Leash ConcaveComponent) -> FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`FermiMatchResult`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Regular(
                                                                FlyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Regular(
                                                                FlyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`Vec Leash ConcaveComponent`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Vec Option Leash ConcaveComponent`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        2,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Vec Leash ConcaveComponent`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`FermiMatchResult`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 23,
                                    src: ExpectationSource {
                                        syn_expr_idx: 24,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`FermiMatchResult`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 24,
                                    src: ExpectationSource {
                                        syn_expr_idx: 25,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`never`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Never,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EthTerm(`FermiMatchResult`),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemPath::AssocItem(
                AssocItemPath::TypeItem(
                    TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::AssocItem(
                    AssocItemPath::TypeItem(
                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::SelfValue(
                                    RegionalTokenIdx(
                                        11,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    owner_ty: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec Leash ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSvar(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`Leash ConcaveComponent`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::SelfValue(
                                    RegionalTokenIdx(
                                        25,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                    owner_ty: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            27,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                11,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    owner_ty: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            32,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                                                    ),
                                                ),
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::max`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`f32`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Regular(
                                                        FlyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FlyTerm {
                                                                place: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`f32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::num::f32(0)::max`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: MutableStackOwned {
                                                            location: PlaceIdx(
                                                                ShiftedU32(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FlyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    13,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
                                                                expectee_place: Leashed,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        7,
                                    ),
                                    opr: Assign,
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    ropd: SemaExprIdx(
                                        14,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            2..5,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`never`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`never`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        15,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFlyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: Some(
                                            ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                        ),
                                        ty_sema_expr_idx: Some(
                                            SemaExprIdx(
                                                1,
                                            ),
                                        ),
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFlyCoersion {
                                                    expectee_place: Const,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                    particulars: SemaForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: SemaExprIdx(
                                            6,
                                        ),
                                        range: SemaForBetweenRange {
                                            initial_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: None,
                                                kind: LowerClosed,
                                            },
                                            final_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        5,
                                                    ),
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    for_loop_var_symbol_idx: 2,
                                    eol_colon: EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                18,
                                            ),
                                        },
                                    ),
                                    block: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            34,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        16,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFlyCoersion {
                                                    expectee_place: MutableStackOwned {
                                                        location: PlaceIdx(
                                                            ShiftedU32(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`never`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    18,
                    (
                        SemaExprIdx(
                            17,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 139,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 139,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        1,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        2,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.0f32`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                MutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`f32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`i32`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    solid_terms: SolTerms {
                        entries: [],
                    },
                    hollow_terms: HolTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: Universe(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                Universe(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        2,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Leashed,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyDerived(
                                    ExpectAnyDerived,
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`never`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Never,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EthTerm(`f32`),
            ),
            self_ty: Some(
                EthTerm(`FermiMatchResult`),
            ),
        },
    },
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemPath::AssocItem(
                AssocItemPath::TypeItem(
                    TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::AssocItem(
                    AssocItemPath::TypeItem(
                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 38,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::SelfValue(
                                    RegionalTokenIdx(
                                        11,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    owner_ty: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec Leash ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSvar(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`Leash ConcaveComponent`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::SelfValue(
                                    RegionalTokenIdx(
                                        25,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                    owner_ty: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            27,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                11,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    owner_ty: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `rel_norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            32,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                                                    ),
                                                ),
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::max`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`f32`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Regular(
                                                        FlyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FlyTerm {
                                                                place: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`f32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::num::f32(0)::max`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: MutableStackOwned {
                                                            location: PlaceIdx(
                                                                ShiftedU32(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FlyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    13,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
                                                                expectee_place: Leashed,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        7,
                                    ),
                                    opr: Assign,
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    ropd: SemaExprIdx(
                                        14,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            2..5,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`never`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`never`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        15,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFlyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: Some(
                                            ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                        ),
                                        ty_sema_expr_idx: Some(
                                            SemaExprIdx(
                                                1,
                                            ),
                                        ),
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFlyCoersion {
                                                    expectee_place: Const,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                    particulars: SemaForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: SemaExprIdx(
                                            6,
                                        ),
                                        range: SemaForBetweenRange {
                                            initial_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: None,
                                                kind: LowerClosed,
                                            },
                                            final_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        5,
                                                    ),
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    for_loop_var_symbol_idx: 2,
                                    eol_colon: EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                18,
                                            ),
                                        },
                                    ),
                                    block: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            34,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        16,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFlyCoersion {
                                                    expectee_place: MutableStackOwned {
                                                        location: PlaceIdx(
                                                            ShiftedU32(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`never`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    18,
                    (
                        SemaExprIdx(
                            17,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 139,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 139,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        1,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        2,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.0f32`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                MutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`f32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`i32`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    solid_terms: SolTerms {
                        entries: [],
                    },
                    hollow_terms: HolTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: Universe(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                Universe(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        2,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Leashed,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyDerived(
                                    ExpectAnyDerived,
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`never`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Never,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EthTerm(`f32`),
            ),
            self_ty: Some(
                EthTerm(`FermiMatchResult`),
            ),
        },
    },
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemPath::AssocItem(
                AssocItemPath::TypeItem(
                    TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::AssocItem(
                    AssocItemPath::TypeItem(
                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::SelfValue(
                                    RegionalTokenIdx(
                                        11,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    owner_ty: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec Leash ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSvar(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`Leash ConcaveComponent`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::SelfValue(
                                    RegionalTokenIdx(
                                        25,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                    owner_ty: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            27,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                11,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    owner_ty: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change`,
                                        regional_token_idx: RegionalTokenIdx(
                                            32,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                                                    ),
                                                ),
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        13,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `abs`,
                                        regional_token_idx: RegionalTokenIdx(
                                            34,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::abs`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`f32`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::num::f32(0)::abs`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::max`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`f32`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Regular(
                                                        FlyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FlyTerm {
                                                                place: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`f32`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`(core::num::f32(0)::max`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: MutableStackOwned {
                                                            location: PlaceIdx(
                                                                ShiftedU32(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FlyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    14,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        7,
                                    ),
                                    opr: Assign,
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    ropd: SemaExprIdx(
                                        15,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: PlaceIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            2..5,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`never`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`never`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        16,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFlyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: Some(
                                            ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                        ),
                                        ty_sema_expr_idx: Some(
                                            SemaExprIdx(
                                                1,
                                            ),
                                        ),
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFlyCoersion {
                                                    expectee_place: Const,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                    particulars: SemaForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: SemaExprIdx(
                                            6,
                                        ),
                                        range: SemaForBetweenRange {
                                            initial_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: None,
                                                kind: LowerClosed,
                                            },
                                            final_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        5,
                                                    ),
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    for_loop_var_symbol_idx: 2,
                                    eol_colon: EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                18,
                                            ),
                                        },
                                    ),
                                    block: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            38,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        17,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFlyCoersion {
                                                    expectee_place: MutableStackOwned {
                                                        location: PlaceIdx(
                                                            ShiftedU32(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`never`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    19,
                    (
                        SemaExprIdx(
                            18,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 139,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 139,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        1,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        2,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.0f32`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                MutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`f32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: PlaceIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`i32`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    solid_terms: SolTerms {
                        entries: [],
                    },
                    hollow_terms: HolTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: Universe(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                Universe(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        2,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyDerived(
                                    ExpectAnyDerived,
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_place: MutableStackOwned {
                                                                location: PlaceIdx(
                                                                    ShiftedU32(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`never`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Never,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EthTerm(`f32`),
            ),
            self_ty: Some(
                EthTerm(`FermiMatchResult`),
            ),
        },
    },
]