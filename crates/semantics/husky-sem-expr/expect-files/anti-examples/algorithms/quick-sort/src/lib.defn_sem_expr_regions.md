```rust
[
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath(`quick_sort::quick_sort`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath(`quick_sort::quick_sort`),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::Variable {
                        current_variable_idx: 0,
                        ident: `len`,
                    },
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodRitchieCall {
                                    self_argument: SemExprIdx(
                                        0,
                                    ),
                                    self_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    self_contract: Contract::Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `len`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                            indirections: [],
                                            final_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        },
                                        signature: MethodFlySignature::TypeMethodRitchie(
                                            TypeMethodRitchieFlySignature {
                                                path: TypeItemPath(
                                                    `core::slice::Slice(0)::len`,
                                                    TypeItemKind::MethodRitchie(
                                                        RitchieItemKind::Fn,
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Contract::Pure,
                                                    ty: FlyTerm {
                                                        quary: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Slice T`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                self_ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Slice T`),
                                                    ),
                                                },
                                                return_ty: FlyTerm {
                                                    quary: Some(
                                                        Transient,
                                                    ),
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`usize`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath(`core::slice::Slice(0)::len`),
                                                    context_itd: EthTermContextItd {
                                                        task_ty: None,
                                                    },
                                                    env: MethodFn {
                                                        self_place: RefMut {
                                                            place: Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                            lifetime: None,
                                                        },
                                                    },
                                                    variable_map: [
                                                        (
                                                            EthSymbolicVariable(`E`, `mono`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    quary: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`E`),
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
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath(`quick_sort::quick_sort_aux`),
                                            context_itd: EthTermContextItd {
                                                task_ty: None,
                                            },
                                            env: TypeOntologyConstructor,
                                            variable_map: [
                                                (
                                                    EthSymbolicVariable(`T`, `mono`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            quary: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        13,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::isize`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `len`,
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        18,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        6,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ropd: SemExprIdx(
                                        7,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Delimitered {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    item: SemExprIdx(
                                        8,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        9,
                                    ),
                                    opr: As,
                                    dispatch: FlyInstanceDispatch {
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
                                    ropd: SemExprIdx(
                                        5,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function: SemExprIdx(
                                        2,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::BorrowMut,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    3,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: RefMut {
                                                                    place: Idx(
                                                                        PlaceIdx(0),
                                                                    ),
                                                                    lifetime: None,
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        12,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    4,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Compterm,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        14,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    10,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            0..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Contract::Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value: SemExprIdx(
                                        1,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        11,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::AnyDerived(
                                            ExpectAnyDerivedOutcome,
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    12,
                    (
                        SemExprIdx(
                            12,
                        ),
                        SynExprRootKind::RootBody,
                    ),
                ),
            ],
            syn_pattern_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            quary: Some(
                                Transient,
                            ),
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 22,
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
            syn_pattern_variable_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 22,
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
            sem_expr_terms: [
                (
                    SemExprIdx(
                        5,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`isize`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        4,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        7,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_variable_map: [
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Type`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                RefMut {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                    lifetime: None,
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Slice T`),
                            ),
                        },
                    ),
                ],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`usize`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_variable_map: [
                    FlyTerm {
                        quary: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`T`),
                        ),
                    },
                ],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`T`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Supertype {
                                            target: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`T`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`T`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::slice::Slice`, `Extern`),
                                    refined_path: Left(
                                        PreludeTypePath::Container(
                                            PreludeContainerTypePath::Slice,
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Slice T`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Ritchie {
                                    ritchie_kind: RitchieKind::Type(
                                        RitchieTypeKind::Item(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    params: [
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::BorrowMut,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                        ),
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((borrow mut Slice T,  isize,  isize) -> unit`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 1,
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
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::AnyDerived,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        quary: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`unit`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::BorrowMut,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::Pure,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`isize`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::Pure,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`isize`),
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::BorrowMut,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: RefMut {
                                                                place: Idx(
                                                                    PlaceIdx(0),
                                                                ),
                                                                lifetime: None,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expectation(
                                            3,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
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
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::TypeOrTrait(
                                    ExpectSortOrTrait,
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::SortOrTrait(
                                                Sort,
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
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Casting(
                                    ExpectCasting {
                                        destination: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ExplicitlyConvertible(
                                                ExpectCastingOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Casting(
                                    ExpectCasting {
                                        destination: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ExplicitlyConvertible(
                                                ExpectCastingOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
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
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
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
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyDerived(
                                                ExpectAnyDerivedOutcome,
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
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyDerived(
                                                ExpectAnyDerivedOutcome,
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
            return_ty: None,
            self_ty: None,
            self_value_ty: None,
            context_itd: EthTermContextItd {
                task_ty: None,
            },
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath(`quick_sort::quick_sort_aux`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath(`quick_sort::quick_sort_aux`),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::Variable {
                        current_variable_idx: 0,
                        ident: `p`,
                    },
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `low`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `low`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        0,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                    ropd: SemExprIdx(
                                        1,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            MajorFormPath(`quick_sort::partition`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath(`quick_sort::partition`),
                                            context_itd: EthTermContextItd {
                                                task_ty: None,
                                            },
                                            env: TypeOntologyConstructor,
                                            variable_map: [
                                                (
                                                    EthSymbolicVariable(`T`, `mono`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            quary: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `low`,
                                    regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `low`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function: SemExprIdx(
                                        3,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::BorrowMut,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    4,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: RefMut {
                                                                    place: Idx(
                                                                        PlaceIdx(0),
                                                                    ),
                                                                    lifetime: None,
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        12,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    5,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(1),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        14,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    6,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(2),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath(`quick_sort::quick_sort_aux`),
                                            context_itd: EthTermContextItd {
                                                task_ty: None,
                                            },
                                            env: TypeOntologyConstructor,
                                            variable_map: [
                                                (
                                                    EthSymbolicVariable(`T`, `mono`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            quary: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    3,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `low`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `low`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `p`,
                                    regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        25,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        11,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    ropd: SemExprIdx(
                                        12,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function: SemExprIdx(
                                        8,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::BorrowMut,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    9,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: RefMut {
                                                                    place: Idx(
                                                                        PlaceIdx(0),
                                                                    ),
                                                                    lifetime: None,
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        20,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    10,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(1),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        22,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    13,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath(`quick_sort::quick_sort_aux`),
                                            context_itd: EthTermContextItd {
                                                task_ty: None,
                                            },
                                            env: TypeOntologyConstructor,
                                            variable_map: [
                                                (
                                                    EthSymbolicVariable(`T`, `mono`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            quary: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    6,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            8,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `p`,
                                    regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    20,
                                    FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        33,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    21,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        17,
                                    ),
                                    opr: Closed(
                                        Add,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    ropd: SemExprIdx(
                                        18,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    22,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    23,
                                    FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function: SemExprIdx(
                                        15,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::BorrowMut,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            7,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    16,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: RefMut {
                                                                    place: Idx(
                                                                        PlaceIdx(0),
                                                                    ),
                                                                    lifetime: None,
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        30,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    19,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        34,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    20,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(2),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    24,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            3..4,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    25,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Contract::Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                    initial_value: SemExprIdx(
                                        7,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        14,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        21,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::AnyDerived(
                                            ExpectAnyDerivedOutcome,
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::IfElse {
                                    if_branch: SemIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                        condition: Other {
                                            expr: SemExprIdx(
                                                2,
                                            ),
                                            conversion: None,
                                        },
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                        stmts: SemStmtIdxRange(
                                            ArenaIdxRange(
                                                0..3,
                                            ),
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    22,
                    (
                        SemExprIdx(
                            22,
                        ),
                        SynExprRootKind::RootBody,
                    ),
                ),
            ],
            syn_pattern_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            quary: None,
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 16,
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
            syn_pattern_variable_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 16,
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
            sem_expr_terms: [
                (
                    SemExprIdx(
                        12,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        18,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_variable_map: [
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Type`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                RefMut {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                    lifetime: None,
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Slice T`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(1),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`isize`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(2),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`isize`),
                            ),
                        },
                    ),
                ],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`isize`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_variable_map: [
                    FlyTerm {
                        quary: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`T`),
                        ),
                    },
                ],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        3,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`T`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Supertype {
                                            target: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`T`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`T`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::slice::Slice`, `Extern`),
                                    refined_path: Left(
                                        PreludeTypePath::Container(
                                            PreludeContainerTypePath::Slice,
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Slice T`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Ritchie {
                                    ritchie_kind: RitchieKind::Type(
                                        RitchieTypeKind::Item(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    params: [
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::BorrowMut,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                        ),
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((borrow mut Slice T,  isize,  isize) -> isize`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        8,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`T`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Supertype {
                                            target: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`T`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`T`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::slice::Slice`, `Extern`),
                                    refined_path: Left(
                                        PreludeTypePath::Container(
                                            PreludeContainerTypePath::Slice,
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Slice T`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Ritchie {
                                    ritchie_kind: RitchieKind::Type(
                                        RitchieTypeKind::Item(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    params: [
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::BorrowMut,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                        ),
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((borrow mut Slice T,  isize,  isize) -> unit`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        15,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`T`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Supertype {
                                            target: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`T`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`T`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::slice::Slice`, `Extern`),
                                    refined_path: Left(
                                        PreludeTypePath::Container(
                                            PreludeContainerTypePath::Slice,
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Slice T`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Ritchie {
                                    ritchie_kind: RitchieKind::Type(
                                        RitchieTypeKind::Item(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    params: [
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::BorrowMut,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            7,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                        ),
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`isize`),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((borrow mut Slice T,  isize,  isize) -> unit`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 7,
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
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: Some(
                                                StackPure {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: StackPure {
                                                                place: Idx(
                                                                    PlaceIdx(2),
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
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::AnyOriginal,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        quary: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`isize`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::BorrowMut,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::Pure,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`isize`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::Pure,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`isize`),
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::BorrowMut,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: RefMut {
                                                                place: Idx(
                                                                    PlaceIdx(0),
                                                                ),
                                                                lifetime: None,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expectation(
                                            4,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
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
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: StackPure {
                                                                place: Idx(
                                                                    PlaceIdx(1),
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: StackPure {
                                                                place: Idx(
                                                                    PlaceIdx(2),
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
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        quary: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`unit`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::BorrowMut,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                4,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::Pure,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`isize`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::Pure,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`isize`),
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::BorrowMut,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    4,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: RefMut {
                                                                place: Idx(
                                                                    PlaceIdx(0),
                                                                ),
                                                                lifetime: None,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    3,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expectation(
                                            10,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: StackPure {
                                                                place: Idx(
                                                                    PlaceIdx(1),
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
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
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
                                        final_destination: FinalDestination::AnyDerived,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        quary: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`unit`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::BorrowMut,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                7,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::Pure,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`isize`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Contract::Pure,
                                                                    ty: FlyTerm {
                                                                        quary: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`isize`),
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::BorrowMut,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: RefMut {
                                                                place: Idx(
                                                                    PlaceIdx(0),
                                                                ),
                                                                lifetime: None,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    6,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expectation(
                                            18,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
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
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 23,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: StackPure {
                                                                place: Idx(
                                                                    PlaceIdx(2),
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
                                expectation: Expectation::AnyDerived(
                                    ExpectAnyDerived,
                                ),
                                state: ExpectationState {
                                    idx: 24,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyDerived(
                                                ExpectAnyDerivedOutcome,
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
                                    idx: 25,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyDerived(
                                                ExpectAnyDerivedOutcome,
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
            return_ty: None,
            self_ty: None,
            self_value_ty: None,
            context_itd: EthTermContextItd {
                task_ty: None,
            },
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath(`quick_sort::partition`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath(`quick_sort::partition`),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::Variable {
                        current_variable_idx: 0,
                        ident: `pivot`,
                    },
                    PlaceInfo::Variable {
                        current_variable_idx: 1,
                        ident: `store_index`,
                    },
                    PlaceInfo::Variable {
                        current_variable_idx: 2,
                        ident: `last_index`,
                    },
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        1,
                                    ),
                                    opr: As,
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ropd: SemExprIdx(
                                        0,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `low`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `low`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        13,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        3,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ropd: SemExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        20,
                                    ),
                                    LiteralTokenData::Bool(
                                        True,
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        24,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        8,
                                    ),
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ropd: SemExprIdx(
                                        9,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        13,
                                    ),
                                    opr: As,
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    ropd: SemExprIdx(
                                        12,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Index {
                                    self_argument: SemExprIdx(
                                        11,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    items: [
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                14,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    index_dynamic_dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                            indirections: [],
                                            final_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`T`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `pivot`,
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Index {
                                    self_argument: SemExprIdx(
                                        16,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    items: [
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                17,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    index_dynamic_dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                            indirections: [],
                                            final_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`T`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        15,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    ropd: SemExprIdx(
                                        18,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    19,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    20,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        40,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    21,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        20,
                                    ),
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ropd: SemExprIdx(
                                        21,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    22,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    23,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        43,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    24,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        23,
                                    ),
                                    opr: AssignClosed(
                                        Sub,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    ropd: SemExprIdx(
                                        24,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    25,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    26,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        47,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    27,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        26,
                                    ),
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    ropd: SemExprIdx(
                                        27,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    28,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    29,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    30,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    31,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        31,
                                    ),
                                    opr: As,
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    ropd: SemExprIdx(
                                        30,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    32,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Index {
                                    self_argument: SemExprIdx(
                                        29,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    items: [
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                32,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                    index_dynamic_dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                            indirections: [],
                                            final_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`T`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    33,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    34,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `pivot`,
                                    regional_token_idx: RegionalTokenIdx(
                                        58,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    35,
                                    FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Index {
                                    self_argument: SemExprIdx(
                                        34,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        57,
                                    ),
                                    items: [
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                35,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                    index_dynamic_dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                            indirections: [],
                                            final_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`T`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    36,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        33,
                                    ),
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    ropd: SemExprIdx(
                                        36,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    37,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        28,
                                    ),
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    ropd: SemExprIdx(
                                        37,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    38,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    39,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        63,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    40,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        39,
                                    ),
                                    opr: AssignClosed(
                                        Sub,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        62,
                                    ),
                                    ropd: SemExprIdx(
                                        40,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    41,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        65,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    42,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        67,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    43,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        42,
                                    ),
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                    ropd: SemExprIdx(
                                        43,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    44,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    45,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    46,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    47,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        47,
                                    ),
                                    opr: As,
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        77,
                                    ),
                                    ropd: SemExprIdx(
                                        46,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    48,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    49,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    50,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        50,
                                    ),
                                    opr: As,
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        81,
                                    ),
                                    ropd: SemExprIdx(
                                        49,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    51,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodRitchieCall {
                                    self_argument: SemExprIdx(
                                        45,
                                    ),
                                    self_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    self_contract: Contract::BorrowMut,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        73,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `swap`,
                                        regional_token_idx: RegionalTokenIdx(
                                            74,
                                        ),
                                    },
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                            indirections: [],
                                            final_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        },
                                        signature: MethodFlySignature::TypeMethodRitchie(
                                            TypeMethodRitchieFlySignature {
                                                path: TypeItemPath(
                                                    `core::slice::Slice(0)::swap`,
                                                    TypeItemKind::MethodRitchie(
                                                        RitchieItemKind::Fn,
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Contract::BorrowMut,
                                                    ty: FlyTerm {
                                                        quary: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Slice T`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Simple(
                                                        FlyRitchieSimpleParameter {
                                                            contract: Contract::Pure,
                                                            ty: FlyTerm {
                                                                quary: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`usize`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                    FlyRitchieParameter::Simple(
                                                        FlyRitchieSimpleParameter {
                                                            contract: Contract::Pure,
                                                            ty: FlyTerm {
                                                                quary: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`usize`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                self_ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Slice T`),
                                                    ),
                                                },
                                                return_ty: FlyTerm {
                                                    quary: Some(
                                                        Transient,
                                                    ),
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`unit`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath(`core::slice::Slice(0)::swap`),
                                                    context_itd: EthTermContextItd {
                                                        task_ty: None,
                                                    },
                                                    env: MethodFn {
                                                        self_place: RefMut {
                                                            place: Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                            lifetime: None,
                                                        },
                                                    },
                                                    variable_map: [
                                                        (
                                                            EthSymbolicVariable(`E`, `mono`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    quary: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`E`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        (
                                                            EthSymbolicVariable(`'self_lifetime`, `nil`),
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
                                        75,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`usize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    48,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        79,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`usize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    51,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        83,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    52,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        RefMut {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                            lifetime: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Slice T`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    53,
                                    FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    54,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        88,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    55,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        55,
                                    ),
                                    opr: As,
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ropd: SemExprIdx(
                                        54,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    56,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    57,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `pivot`,
                                    regional_token_idx: RegionalTokenIdx(
                                        92,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    58,
                                    FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        58,
                                    ),
                                    opr: As,
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        93,
                                    ),
                                    ropd: SemExprIdx(
                                        57,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`usize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    59,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodRitchieCall {
                                    self_argument: SemExprIdx(
                                        53,
                                    ),
                                    self_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    self_contract: Contract::BorrowMut,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        85,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `swap`,
                                        regional_token_idx: RegionalTokenIdx(
                                            86,
                                        ),
                                    },
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                            indirections: [],
                                            final_place: RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        },
                                        signature: MethodFlySignature::TypeMethodRitchie(
                                            TypeMethodRitchieFlySignature {
                                                path: TypeItemPath(
                                                    `core::slice::Slice(0)::swap`,
                                                    TypeItemKind::MethodRitchie(
                                                        RitchieItemKind::Fn,
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Contract::BorrowMut,
                                                    ty: FlyTerm {
                                                        quary: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Slice T`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Simple(
                                                        FlyRitchieSimpleParameter {
                                                            contract: Contract::Pure,
                                                            ty: FlyTerm {
                                                                quary: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`usize`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                    FlyRitchieParameter::Simple(
                                                        FlyRitchieSimpleParameter {
                                                            contract: Contract::Pure,
                                                            ty: FlyTerm {
                                                                quary: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`usize`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                self_ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Slice T`),
                                                    ),
                                                },
                                                return_ty: FlyTerm {
                                                    quary: Some(
                                                        Transient,
                                                    ),
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`unit`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath(`core::slice::Slice(0)::swap`),
                                                    context_itd: EthTermContextItd {
                                                        task_ty: None,
                                                    },
                                                    env: MethodFn {
                                                        self_place: RefMut {
                                                            place: Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                            lifetime: None,
                                                        },
                                                    },
                                                    variable_map: [
                                                        (
                                                            EthSymbolicVariable(`E`, `mono`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    quary: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`E`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        (
                                                            EthSymbolicVariable(`'self_lifetime`, `nil`),
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
                                        87,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`usize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    56,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        91,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Contract::Pure,
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`usize`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    59,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    60,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        MutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    61,
                                    FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            9..15,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    62,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        22,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        41,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Break {
                                    break_token: BreakRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            69,
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`never`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        52,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        10,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::While {
                                    while_token: WhileRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            25,
                                        ),
                                    },
                                    condition: Other {
                                        expr: SemExprIdx(
                                            19,
                                        ),
                                        conversion: None,
                                    },
                                    eol_colon: EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                37,
                                            ),
                                        },
                                    ),
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            0..1,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        25,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::While {
                                    while_token: WhileRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            44,
                                        ),
                                    },
                                    condition: Other {
                                        expr: SemExprIdx(
                                            38,
                                        ),
                                        conversion: None,
                                    },
                                    eol_colon: EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                60,
                                            ),
                                        },
                                    ),
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::IfElse {
                                    if_branch: SemIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                64,
                                            ),
                                        },
                                        condition: Other {
                                            expr: SemExprIdx(
                                                44,
                                            ),
                                            conversion: None,
                                        },
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                68,
                                            ),
                                        },
                                        stmts: SemStmtIdxRange(
                                            ArenaIdxRange(
                                                2..3,
                                            ),
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: Some(
                                        SemElseBranch {
                                            else_regional_token: ElseRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    70,
                                                ),
                                            },
                                            eol_colon_regional_token: EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    71,
                                                ),
                                            },
                                            stmts: SemStmtIdxRange(
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Contract::Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value: SemExprIdx(
                                        2,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Contract::Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            10,
                                        ),
                                    ),
                                    initial_value: SemExprIdx(
                                        5,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Contract::Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            17,
                                        ),
                                    ),
                                    initial_value: SemExprIdx(
                                        6,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::While {
                                    while_token: WhileRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                    },
                                    condition: Other {
                                        expr: SemExprIdx(
                                            7,
                                        ),
                                        conversion: None,
                                    },
                                    eol_colon: EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                21,
                                            ),
                                        },
                                    ),
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            4..9,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        60,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    expr: SemExprIdx(
                                        61,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: MutableOnStack {
                                                            place: Idx(
                                                                PlaceIdx(1),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`isize`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    62,
                    (
                        SemExprIdx(
                            62,
                        ),
                        SynExprRootKind::RootBody,
                    ),
                ),
            ],
            syn_pattern_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            quary: None,
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 22,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            quary: Some(
                                Transient,
                            ),
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            quary: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(2),
                                    ),
                                },
                            ),
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 16,
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
            syn_pattern_variable_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 22,
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
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 16,
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
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 16,
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
            sem_expr_terms: [
                (
                    SemExprIdx(
                        0,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`usize`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        12,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`usize`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        30,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`usize`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        46,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`usize`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        49,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`usize`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        54,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`usize`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        57,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`usize`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        4,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        7,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`true`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        9,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        21,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        24,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        27,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        40,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_variable_map: [
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Type`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                RefMut {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                    lifetime: None,
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Slice T`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(1),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`isize`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(2),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`isize`),
                            ),
                        },
                    ),
                ],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`usize`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                MutableOnStack {
                                    place: Idx(
                                        PlaceIdx(1),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`isize`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                MutableOnStack {
                                    place: Idx(
                                        PlaceIdx(2),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`isize`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_variable_map: [
                    FlyTerm {
                        quary: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`T`),
                        ),
                    },
                ],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::TypeOrTrait(
                                    ExpectSortOrTrait,
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::SortOrTrait(
                                                Sort,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Casting(
                                    ExpectCasting {
                                        destination: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ExplicitlyConvertible(
                                                ExpectCastingOutcome,
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
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: Some(
                                                StackPure {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
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
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
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
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::TypeOrTrait(
                                    ExpectSortOrTrait,
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::SortOrTrait(
                                                Sort,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Casting(
                                    ExpectCasting {
                                        destination: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ExplicitlyConvertible(
                                                ExpectCastingOutcome,
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
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: Some(
                                                RefMut {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                    lifetime: None,
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`T`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: RefMut {
                                                                place: Idx(
                                                                    PlaceIdx(0),
                                                                ),
                                                                lifetime: None,
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
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
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
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
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
                                    idx: 23,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 24,
                                    src: ExpectationSource {
                                        syn_expr_idx: 24,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 25,
                                    src: ExpectationSource {
                                        syn_expr_idx: 25,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
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
                                    idx: 26,
                                    src: ExpectationSource {
                                        syn_expr_idx: 26,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: Some(
                                                MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 27,
                                    src: ExpectationSource {
                                        syn_expr_idx: 27,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 28,
                                    src: ExpectationSource {
                                        syn_expr_idx: 36,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
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
                                    idx: 29,
                                    src: ExpectationSource {
                                        syn_expr_idx: 28,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::TypeOrTrait(
                                    ExpectSortOrTrait,
                                ),
                                state: ExpectationState {
                                    idx: 30,
                                    src: ExpectationSource {
                                        syn_expr_idx: 30,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::SortOrTrait(
                                                Sort,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Casting(
                                    ExpectCasting {
                                        destination: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 31,
                                    src: ExpectationSource {
                                        syn_expr_idx: 29,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ExplicitlyConvertible(
                                                ExpectCastingOutcome,
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
                                    idx: 32,
                                    src: ExpectationSource {
                                        syn_expr_idx: 31,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 33,
                                    src: ExpectationSource {
                                        syn_expr_idx: 34,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 34,
                                    src: ExpectationSource {
                                        syn_expr_idx: 32,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 35,
                                    src: ExpectationSource {
                                        syn_expr_idx: 33,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: Some(
                                                RefMut {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                    lifetime: None,
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`T`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 36,
                                    src: ExpectationSource {
                                        syn_expr_idx: 35,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: RefMut {
                                                                place: Idx(
                                                                    PlaceIdx(0),
                                                                ),
                                                                lifetime: None,
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 37,
                                    src: ExpectationSource {
                                        syn_expr_idx: 37,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 38,
                                    src: ExpectationSource {
                                        syn_expr_idx: 38,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
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
                                    idx: 39,
                                    src: ExpectationSource {
                                        syn_expr_idx: 39,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 40,
                                    src: ExpectationSource {
                                        syn_expr_idx: 40,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 41,
                                    src: ExpectationSource {
                                        syn_expr_idx: 41,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
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
                                    idx: 42,
                                    src: ExpectationSource {
                                        syn_expr_idx: 42,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: Some(
                                                MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 43,
                                    src: ExpectationSource {
                                        syn_expr_idx: 43,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: MutableOnStack {
                                                                place: Idx(
                                                                    PlaceIdx(2),
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
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 44,
                                    src: ExpectationSource {
                                        syn_expr_idx: 44,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
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
                                    idx: 45,
                                    src: ExpectationSource {
                                        syn_expr_idx: 45,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::TypeOrTrait(
                                    ExpectSortOrTrait,
                                ),
                                state: ExpectationState {
                                    idx: 46,
                                    src: ExpectationSource {
                                        syn_expr_idx: 47,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::SortOrTrait(
                                                Sort,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Casting(
                                    ExpectCasting {
                                        destination: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 47,
                                    src: ExpectationSource {
                                        syn_expr_idx: 46,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ExplicitlyConvertible(
                                                ExpectCastingOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 48,
                                    src: ExpectationSource {
                                        syn_expr_idx: 48,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::TypeOrTrait(
                                    ExpectSortOrTrait,
                                ),
                                state: ExpectationState {
                                    idx: 49,
                                    src: ExpectationSource {
                                        syn_expr_idx: 50,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::SortOrTrait(
                                                Sort,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Casting(
                                    ExpectCasting {
                                        destination: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 50,
                                    src: ExpectationSource {
                                        syn_expr_idx: 49,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ExplicitlyConvertible(
                                                ExpectCastingOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 51,
                                    src: ExpectationSource {
                                        syn_expr_idx: 51,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 52,
                                    src: ExpectationSource {
                                        syn_expr_idx: 52,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
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
                                    idx: 53,
                                    src: ExpectationSource {
                                        syn_expr_idx: 53,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            RefMut {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                                lifetime: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Slice T`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::TypeOrTrait(
                                    ExpectSortOrTrait,
                                ),
                                state: ExpectationState {
                                    idx: 54,
                                    src: ExpectationSource {
                                        syn_expr_idx: 55,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::SortOrTrait(
                                                Sort,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Casting(
                                    ExpectCasting {
                                        destination: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 55,
                                    src: ExpectationSource {
                                        syn_expr_idx: 54,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ExplicitlyConvertible(
                                                ExpectCastingOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 56,
                                    src: ExpectationSource {
                                        syn_expr_idx: 56,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::TypeOrTrait(
                                    ExpectSortOrTrait,
                                ),
                                state: ExpectationState {
                                    idx: 57,
                                    src: ExpectationSource {
                                        syn_expr_idx: 58,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::SortOrTrait(
                                                Sort,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Casting(
                                    ExpectCasting {
                                        destination: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 58,
                                    src: ExpectationSource {
                                        syn_expr_idx: 57,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ExplicitlyConvertible(
                                                ExpectCastingOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 59,
                                    src: ExpectationSource {
                                        syn_expr_idx: 59,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`usize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Pure,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 60,
                                    src: ExpectationSource {
                                        syn_expr_idx: 60,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 61,
                                    src: ExpectationSource {
                                        syn_expr_idx: 61,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            MutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: MutableOnStack {
                                                                place: Idx(
                                                                    PlaceIdx(1),
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`isize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 62,
                                    src: ExpectationSource {
                                        syn_expr_idx: 62,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`isize`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
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
                EthTerm(`isize`),
            ),
            self_ty: None,
            self_value_ty: None,
            context_itd: EthTermContextItd {
                task_ty: None,
            },
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath(`quick_sort::quick_sort_works_for_integers`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath(`quick_sort::quick_sort_works_for_integers`),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::Variable {
                        current_variable_idx: 0,
                        ident: `v`,
                    },
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::VecFunctor {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`covariant Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`covariant Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::i32`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionApplication {
                                    function: SemExprIdx(
                                        0,
                                    ),
                                    argument: SemExprIdx(
                                        1,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            4,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        12,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            65,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        17,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            31,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    opd: SemExprIdx(
                                        6,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        19,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        21,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            99,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        23,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        25,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            83,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
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
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        27,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            782,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        29,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
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
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::NewList {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    items: [
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                3,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    11,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                4,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                5,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    15,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                7,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    18,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                8,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    20,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                9,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    22,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                10,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    24,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                11,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    26,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                12,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    28,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                13,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    element_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            0..1,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: Some(
                                            ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                        ),
                                        ty_sem_expr_idx: Some(
                                            SemExprIdx(
                                                2,
                                            ),
                                        ),
                                    },
                                    contract: Contract::Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                    initial_value: SemExprIdx(
                                        14,
                                    ),
                                    coercion_outcome: Some(
                                        ExpectCoercionOutcome {
                                            coercion: Trivial(
                                                TrivialFlyCoercion {
                                                    expectee_quary: Transient,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    15,
                    (
                        SemExprIdx(
                            15,
                        ),
                        SynExprRootKind::RootBody,
                    ),
                ),
            ],
            syn_pattern_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            quary: Some(
                                Transient,
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_variable_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: Eth(
                                        Application(
                                            EthApplication(
                                                Id {
                                                    value: 5,
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
            sem_expr_terms: [
                (
                    SemExprIdx(
                        0,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        1,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`i32`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        2,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec i32`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        3,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`4`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        4,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`65`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        5,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`2`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        6,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`31`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        8,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        9,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`99`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        10,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`2`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        11,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`83`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        12,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`782`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        13,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_variable_map: [],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                MutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec i32`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_variable_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        6,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`i32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`i32`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 1,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::Sort,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`covariant Type -> Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        quary: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Type`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::ExplicitCurry {
                                                        variance: Covariant,
                                                        parameter_hvar: None,
                                                        parameter_ty: FlyTerm {
                                                            quary: None,
                                                            base: FlyTermBase::Eth(
                                                                EthTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FlyTerm {
                                                            quary: None,
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
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Type`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectSort {
                                        smallest_universe: Universe(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
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
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
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
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Vec i32`),
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
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
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
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyDerived(
                                                ExpectAnyDerivedOutcome,
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
            return_ty: None,
            self_ty: None,
            self_value_ty: None,
            context_itd: EthTermContextItd {
                task_ty: None,
            },
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath(`quick_sort::quick_sort_works_for_strs`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath(`quick_sort::quick_sort_works_for_strs`),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::Variable {
                        current_variable_idx: 0,
                        ident: `strs`,
                    },
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "beach",
                                        },
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Ref 'static str`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "hotel",
                                        },
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Ref 'static str`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "airplane",
                                        },
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Ref 'static str`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        12,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "car",
                                        },
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Ref 'static str`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "house",
                                        },
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Ref 'static str`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        16,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "art",
                                        },
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Ref 'static str`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::NewList {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    items: [
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                0,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                1,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                2,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    11,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                3,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                4,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    15,
                                                ),
                                            ),
                                        },
                                        SemCommaListItem {
                                            expr: SemExprIdx(
                                                5,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    element_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Ref 'static str`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Ref 'static str`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            0..1,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Contract::Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    initial_value: SemExprIdx(
                                        6,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    7,
                    (
                        SemExprIdx(
                            7,
                        ),
                        SynExprRootKind::RootBody,
                    ),
                ),
            ],
            syn_pattern_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            quary: Some(
                                Transient,
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_variable_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: Eth(
                                        Application(
                                            EthApplication(
                                                Id {
                                                    value: 6,
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
            sem_expr_terms: [
                (
                    SemExprIdx(
                        0,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`"beach"`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        1,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`"hotel"`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        2,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`"airplane"`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        3,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`"car"`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        4,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`"house"`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        5,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`"art"`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_variable_map: [],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            quary: Some(
                                MutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec Ref 'static str`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_variable_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        6,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            quary: Some(
                                                Compterm,
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Ref 'static str`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                quary: Some(
                                                    Compterm,
                                                ),
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Ref 'static str`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Ref 'static str`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 1,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
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
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Contract::Move,
                                        ty_expected: FlyTerm {
                                            quary: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Ref 'static str`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Ref 'static str`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
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
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyDerived(
                                                ExpectAnyDerivedOutcome,
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
            return_ty: None,
            self_ty: None,
            self_value_ty: None,
            context_itd: EthTermContextItd {
                task_ty: None,
            },
        },
    },
]
```