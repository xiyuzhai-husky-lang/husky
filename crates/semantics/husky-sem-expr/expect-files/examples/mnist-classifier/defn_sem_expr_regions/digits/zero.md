```rust
[
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [],
                next: 0,
            },
            sem_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                                        Fn,
                                                    )`),
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
                                        EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
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
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
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
                                            Leashed {
                                                place_idx: None,
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                                                        Fn,
                                                    )`),
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
                                        EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::NewList {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                2,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    element_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemaExprIdx(
                                        0,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Leash Vec ConcaveComponent`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemaExprIdx(
                                                    1,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: None,
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemaExprIdx(
                                                    3,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
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
                                        8,
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
                                    4,
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
                                            0..1,
                                        ),
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
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sem_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFlyCoersion {
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
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    5,
                    (
                        SemaExprIdx(
                            5,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
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
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                        ),
                                    },
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
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Pure,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`Leash Vec ConcaveComponent`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Pure,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Leash Vec ConcaveComponent`),
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
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Leashed {
                                                                place_idx: None,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
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
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
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
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                expectation: Expectation::Coersion(
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
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                expectation: Expectation::Coersion(
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                EthTerm(`FermiMatchResult`),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                            Fn,
                        )`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [],
                next: 0,
            },
            sem_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_syn_symbol_idx: 0,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
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
                                    self_argument: SemaExprIdx(
                                        0,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change`,
                                        regional_token_idx: RegionalTokenIdx(
                                            4,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
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
                                                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::angle_change`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::angle_change`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 40,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        1,
                                    ),
                                    opr: Closed(
                                        Add,
                                    ),
                                    dispatch: FlyDynamicDispatch {
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
                                    ropd: SemaExprIdx(
                                        2,
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
                                    3,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 41,
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
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: Some(
                                            Const,
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
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    opd: SemaExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
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
                                    5,
                                    FlyTerm {
                                        place: Some(
                                            Const,
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
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        3,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ropd: SemaExprIdx(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    inherited_syn_symbol_idx: 0,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
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
                                    self_argument: SemaExprIdx(
                                        7,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change`,
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
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
                                                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::angle_change`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::angle_change`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
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
                                            Leashed {
                                                place_idx: None,
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
                                SemaExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    opd: SemaExprIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        15,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 42,
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
                                    10,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        9,
                                    ),
                                    opr: Closed(
                                        Add,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    ropd: SemaExprIdx(
                                        10,
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
                                    11,
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
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            0..2,
                                        ),
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
                                    12,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            6,
                                        ),
                                        conversion: None,
                                    },
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
                                    sem_expr_idx: SemaExprIdx(
                                        11,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: WrapInSome,
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
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
                        SemaExprIdx(
                            12,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [
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
                (
                    SemaExprIdx(
                        4,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`140.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        10,
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
                inherited_syn_symbol_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [],
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
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        3,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`f32`),
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
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
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
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                Leashed {
                                                    place_idx: None,
                                                },
                                            ),
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
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
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
                                expectation: Expectation::Coersion(
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
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
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                Leashed {
                                                    place_idx: None,
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
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
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Option f32`),
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
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: WrapInSome,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Option f32`),
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
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: WrapInSome,
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
                EthTerm(`Option f32`),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    Variable {
                        current_syn_symbol_idx: 0,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 288,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 1,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 291,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 2,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 293,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 3,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 297,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 4,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 86,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 5,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 119,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 6,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 299,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 7,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 86,
                                },
                            ),
                        ),
                    },
                ],
                next: 8,
            },
            sem_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        0,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `raw_contours`,
                                        regional_token_idx: RegionalTokenIdx(
                                            4,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec RawContour`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::raw_contours`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::raw_contours`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec RawContour`),
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
                                                            TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSymbolicVariable(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`RawContour`),
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
                                    2,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
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
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: Some(
                                            Const,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        2,
                                    ),
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ropd: SemaExprIdx(
                                        3,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
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
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
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
                                    self_argument: SemaExprIdx(
                                        5,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
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
                                    ident: `n`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
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
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        21,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 43,
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
                                    8,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        7,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
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
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
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
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
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
                                    self_argument: SemaExprIdx(
                                        10,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            25,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        27,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
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
                                    12,
                                    FlyTerm {
                                        place: Some(
                                            Const,
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
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner: SemaExprIdx(
                                        11,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                12,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Be {
                                    src: SemaExprIdx(
                                        13,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    target: BePatternSyndicate {
                                        pattern_expr_root: BeSynPatternExprRoot {
                                            syn_pattern_expr_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            1..1,
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec ConnectedComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec ConnectedComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        15,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            37,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec ConnectedComponent`),
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
                                                            TypeItemPath(`<core::vec::Vec(0)>::ilen`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSymbolicVariable(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`ConnectedComponent`),
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
                                        38,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        39,
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
                                    17,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        41,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
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
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FlyTerm {
                                        place: Some(
                                            Const,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        16,
                                    ),
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                    ropd: SemaExprIdx(
                                        17,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    19,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    20,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
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
                                    self_argument: SemaExprIdx(
                                        19,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            47,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
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
                                            Leashed {
                                                place_idx: None,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        49,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    22,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner: SemaExprIdx(
                                        20,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                21,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    24,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sem_expr_idx: SemaExprIdx(
                                        22,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    25,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        23,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            53,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::displacement`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::displacement`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
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
                                        54,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    26,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        24,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            57,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::norm`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vector2d`),
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
                                                            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::norm`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Transient,
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
                                        58,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        59,
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
                                    27,
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
                                SemaExprData::CurrentSynSymbol {
                                    ident: `c`,
                                    regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
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
                                    28,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        63,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 44,
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
                                    29,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        26,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FlyDynamicDispatch {
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
                                    ropd: SemaExprIdx(
                                        27,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    30,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 7,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 252,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::TypeVariant(
                                                Room32,
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 252,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`Label`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EthSymbolicVariable(`label`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
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
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    31,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 8,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                                        Fn,
                                                    )`),
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
                                        EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    34,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 9,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    35,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
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
                                SemaExprData::NewList {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        75,
                                    ),
                                    items: [],
                                    element_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    36,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemaExprIdx(
                                        30,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Leash Vec ConcaveComponent`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemaExprIdx(
                                                    31,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: None,
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        74,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemaExprIdx(
                                                    32,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFlyCoersion {
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
                                        77,
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
                                    37,
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 10,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`malamute::narrow_down`, `Ritchie(
                                                Gn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Ritchie(
                                                        Gn,
                                                    )`),
                                                ),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`Label`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    5,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EthSymbolicVariable(`label`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
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
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            8,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    38,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    39,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
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
                                    self_argument: SemaExprIdx(
                                        35,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        81,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            82,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
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
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
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
                                    40,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
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
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    41,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
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
                                    self_argument: SemaExprIdx(
                                        37,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        85,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `rel_norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            86,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::rel_norm`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::rel_norm`, `MemoizedField`),
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
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
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
                                    42,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
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
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        88,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    43,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
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
                                    self_argument: SemaExprIdx(
                                        39,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change_norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            90,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::angle_change_norm`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::angle_change_norm`, `MemoizedField`),
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
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
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
                                    44,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        94,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            5,
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
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    45,
                                    FlyTerm {
                                        place: Some(
                                            Const,
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
                                SemaExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemaExprIdx(
                                        34,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Gn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Variadic(
                                            FlyRitchieVariadicParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                            },
                                            [
                                                SemaVariadicCallListItem {
                                                    argument_sem_expr_idx: SemaExprIdx(
                                                        36,
                                                    ),
                                                    coersion_outcome: Some(
                                                        ExpectCoersionOutcome {
                                                            coersion: Trivial(
                                                                TrivialFlyCoersion {
                                                                    expectee_quary: ImmutableOnStack {
                                                                        place: Idx(
                                                                            PlaceIdx(2),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    separator: CallListSeparator::Comma(
                                                        RegionalTokenIdx(
                                                            83,
                                                        ),
                                                    ),
                                                },
                                                SemaVariadicCallListItem {
                                                    argument_sem_expr_idx: SemaExprIdx(
                                                        38,
                                                    ),
                                                    coersion_outcome: Some(
                                                        ExpectCoersionOutcome {
                                                            coersion: Trivial(
                                                                TrivialFlyCoersion {
                                                                    expectee_quary: ImmutableOnStack {
                                                                        place: Idx(
                                                                            PlaceIdx(2),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    separator: CallListSeparator::Comma(
                                                        RegionalTokenIdx(
                                                            87,
                                                        ),
                                                    ),
                                                },
                                                SemaVariadicCallListItem {
                                                    argument_sem_expr_idx: SemaExprIdx(
                                                        40,
                                                    ),
                                                    coersion_outcome: Some(
                                                        ExpectCoersionOutcome {
                                                            coersion: Trivial(
                                                                TrivialFlyCoersion {
                                                                    expectee_quary: ImmutableOnStack {
                                                                        place: Idx(
                                                                            PlaceIdx(2),
                                                                        ),
                                                                    },
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
                                            ],
                                        ),
                                        SemaRitchieArgument::Keyed(
                                            FlyRitchieKeyedParameter {
                                                key: `skip`,
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                has_default: true,
                                            },
                                            Some(
                                                SemaKeyedCallListItem {
                                                    key_regional_token_idx: RegionalTokenIdx(
                                                        92,
                                                    ),
                                                    key: `skip`,
                                                    argument_sem_expr_idx: SemaExprIdx(
                                                        41,
                                                    ),
                                                    coersion_outcome: Some(
                                                        ExpectCoersionOutcome {
                                                            coersion: Trivial(
                                                                TrivialFlyCoersion {
                                                                    expectee_quary: Const,
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    separator: CallListSeparator::None,
                                                },
                                            ),
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    46,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unveil {
                                    opd_sem_expr_idx: SemaExprIdx(
                                        42,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                    unveil_output_ty_signature: TraitForTypeAssocTypeEtherealSignature {
                                        path: TraitForTypeItemPath(
                                            `<malamute::OneVsAll as core::ops::Unveil(0)>::Output`,
                                            TraitItemKind::AssocType,
                                        ),
                                        instantiation: EthInstantiation {
                                            path: ItemPath::ImplBlock(
                                                ImplBlockPath::TraitForTypeImplBlock(
                                                    TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                                ),
                                            ),
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`Label`),
                                                    EthTerm(`MnistLabel`),
                                                ),
                                                (
                                                    EthSymbolicVariable(`label`),
                                                    EthTerm(`Zero`),
                                                ),
                                            ],
                                            separator: Some(
                                                2,
                                            ),
                                        },
                                        ty_term: EthTerm(`unit`),
                                    },
                                    unveil_assoc_fn_path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    return_ty: EthTerm(`OneVsAll MnistLabel Zero`),
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
                                    49,
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
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        98,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    50,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
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
                                    self_argument: SemaExprIdx(
                                        44,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        99,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            100,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
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
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
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
                                    51,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        102,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 45,
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
                                    52,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        45,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                    ropd: SemaExprIdx(
                                        46,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    53,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 11,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    54,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        48,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `eff_holes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            106,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`EffHoles`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`EffHoles`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    55,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`EffHoles`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        49,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`EffHoles`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        107,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            108,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Option Leash RawContour`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    56,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        110,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    57,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner: SemaExprIdx(
                                        50,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                51,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        111,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Option Leash RawContour`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    59,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Be {
                                    src: SemaExprIdx(
                                        52,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        112,
                                    ),
                                    target: BePatternSyndicate {
                                        pattern_expr_root: BeSynPatternExprRoot {
                                            syn_pattern_expr_idx: 5,
                                        },
                                        variables: ArenaIdxRange(
                                            3..3,
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    60,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 13,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    61,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        54,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        116,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `eff_holes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            117,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`EffHoles`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`EffHoles`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    62,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`EffHoles`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        55,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`EffHoles`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        118,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            119,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Option Leash RawContour`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    63,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        121,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    64,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner: SemaExprIdx(
                                        56,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        120,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                57,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        122,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Option Leash RawContour`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    66,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Be {
                                    src: SemaExprIdx(
                                        58,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        123,
                                    ),
                                    target: BePatternSyndicate {
                                        pattern_expr_root: BeSynPatternExprRoot {
                                            syn_pattern_expr_idx: 7,
                                        },
                                        variables: ArenaIdxRange(
                                            3..3,
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    67,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 15,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    68,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        60,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        132,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `eff_holes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            133,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`EffHoles`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`EffHoles`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    69,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`EffHoles`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        61,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`EffHoles`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        134,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            135,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Option Leash RawContour`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    70,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        137,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            11,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    71,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                11,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner: SemaExprIdx(
                                        62,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        136,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                63,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        138,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Option Leash RawContour`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    73,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `major_hole`,
                                    regional_token_idx: RegionalTokenIdx(
                                        142,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(3),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    74,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sem_expr_idx: SemaExprIdx(
                                        65,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        143,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    75,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        66,
                                    ),
                                    self_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash RawContour`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        144,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            145,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::bounding_box`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::bounding_box`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    76,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        67,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        146,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
                                        regional_token_idx: RegionalTokenIdx(
                                            147,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymax`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`BoundingBox`),
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
                                                            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymax`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
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
                                        148,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        149,
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
                                    77,
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
                                SemaExprData::CurrentSynSymbol {
                                    ident: `major_hole`,
                                    regional_token_idx: RegionalTokenIdx(
                                        151,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(3),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    78,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sem_expr_idx: SemaExprIdx(
                                        69,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        152,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    79,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        70,
                                    ),
                                    self_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash RawContour`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        153,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            154,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::bounding_box`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::bounding_box`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    80,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        71,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        155,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            156,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymin`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`BoundingBox`),
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
                                                            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymin`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
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
                                        157,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        158,
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
                                    81,
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
                                        68,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        150,
                                    ),
                                    ropd: SemaExprIdx(
                                        72,
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
                                    82,
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 16,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    83,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        74,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash LineSegmentSketch`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        163,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            164,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::bounding_box`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::bounding_box`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    84,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        75,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        165,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
                                        regional_token_idx: RegionalTokenIdx(
                                            166,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymax`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`BoundingBox`),
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
                                                            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymax`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
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
                                        167,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        168,
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
                                    85,
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 17,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    86,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        77,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash LineSegmentSketch`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        171,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            172,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::bounding_box`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch(0)>::bounding_box`, `MemoizedField`),
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    87,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        78,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        173,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            174,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymin`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`BoundingBox`),
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
                                                            TypeItemPath(`<mnist_classifier::geom2d::BoundingBox(0)>::ymin`, `MethodRitchie(
                                                                Fn,
                                                            )`),
                                                        ),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
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
                                        175,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        176,
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
                                    88,
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
                                        76,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        169,
                                    ),
                                    ropd: SemaExprIdx(
                                        79,
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
                                    89,
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
                                SemaExprData::CurrentSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        180,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(4),
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
                                    90,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(4),
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
                                    ident: `b`,
                                    regional_token_idx: RegionalTokenIdx(
                                        182,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(5),
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
                                    91,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(5),
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        81,
                                    ),
                                    opr: Closed(
                                        Div,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        181,
                                    ),
                                    ropd: SemaExprIdx(
                                        82,
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
                                    92,
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
                                SemaExprData::CurrentSynSymbol {
                                    ident: `ratio`,
                                    regional_token_idx: RegionalTokenIdx(
                                        184,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(6),
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
                                    93,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(6),
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        186,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 46,
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
                                    94,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        84,
                                    ),
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        185,
                                    ),
                                    ropd: SemaExprIdx(
                                        85,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    95,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        190,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    96,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
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
                                    self_argument: SemaExprIdx(
                                        87,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        191,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            192,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
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
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
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
                                    97,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 19,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 252,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::TypeVariant(
                                                Room32,
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 252,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`Label`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    12,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EthSymbolicVariable(`label`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    13,
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
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            14,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    98,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                14,
                                            ),
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
                                            7..20,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            14,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    101,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                14,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            14,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        6,
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            9,
                                        ),
                                        conversion: None,
                                    },
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    condition: Be {
                                        src: SemaExprIdx(
                                            13,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            29,
                                        ),
                                        target: BePatternSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                1..1,
                                            ),
                                        },
                                    },
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            34,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            18,
                                        ),
                                        conversion: None,
                                    },
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
                                            42,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 3,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            44,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        25,
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            60,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            28,
                                        ),
                                        conversion: None,
                                    },
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
                                            64,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        29,
                                    ),
                                    coersion_outcome: Some(
                                        ExpectCoersionOutcome {
                                            coersion: Trivial(
                                                TrivialFlyCoersion {
                                                    expectee_quary: Transient,
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
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::IfElse {
                                    if_branch: SemaIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                        condition: Other {
                                            sem_expr_idx: SemaExprIdx(
                                                4,
                                            ),
                                            conversion: None,
                                        },
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                11,
                                            ),
                                        },
                                        stmts: SemaStmtIdxRange(
                                            ArenaIdxRange(
                                                0..7,
                                            ),
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
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
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            68,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 4,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            70,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        33,
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
                                    sem_expr_idx: SemaExprIdx(
                                        43,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFlyCoersion {
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
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            97,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            47,
                                        ),
                                        conversion: None,
                                    },
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            103,
                                        ),
                                    },
                                    condition: Be {
                                        src: SemaExprIdx(
                                            52,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            112,
                                        ),
                                        target: BePatternSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 5,
                                            },
                                            variables: ArenaIdxRange(
                                                3..3,
                                            ),
                                        },
                                    },
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            114,
                                        ),
                                    },
                                    condition: Be {
                                        src: SemaExprIdx(
                                            58,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            123,
                                        ),
                                        target: BePatternSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 7,
                                            },
                                            variables: ArenaIdxRange(
                                                3..3,
                                            ),
                                        },
                                    },
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
                                            128,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 8,
                                        },
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            130,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        64,
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
                                            139,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 9,
                                        },
                                        variables: ArenaIdxRange(
                                            4..5,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            141,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        73,
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
                                            159,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 10,
                                        },
                                        variables: ArenaIdxRange(
                                            5..6,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            161,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        80,
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
                                            177,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 11,
                                        },
                                        variables: ArenaIdxRange(
                                            6..7,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            179,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        83,
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            183,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            86,
                                        ),
                                        conversion: None,
                                    },
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
                                            187,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 12,
                                        },
                                        variables: ArenaIdxRange(
                                            7..8,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            189,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        88,
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
                                    sem_expr_idx: SemaExprIdx(
                                        89,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFlyCoersion {
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
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            14,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    90,
                    (
                        SemaExprIdx(
                            90,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                Leashed {
                                    place_idx: None,
                                },
                            ),
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
                PatternExprTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                Leashed {
                                    place_idx: None,
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 58,
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
                                                    value: 52,
                                                },
                                            ),
                                        ),
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
                                Leashed {
                                    place_idx: None,
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 41,
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
                                Leashed {
                                    place_idx: None,
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 41,
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
                                Leashed {
                                    place_idx: None,
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 41,
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
                PatternExprTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(2),
                                    ),
                                },
                            ),
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
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
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
                    None,
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
                                                            value: 52,
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
                    None,
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: Eth(
                                        Application(
                                            EthApplication(
                                                Id {
                                                    value: 41,
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
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(2),
                                            ),
                                        },
                                    ),
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
            sem_expr_terms: [
                (
                    SemaExprIdx(
                        3,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        8,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1.5f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        12,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        17,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        21,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        27,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`5.5f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        41,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`5`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        46,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`3.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        51,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        57,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        63,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        85,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.4f32`),
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
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
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
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(1),
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
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(2),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`FermiMatchResult`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(3),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Option Leash RawContour`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(4),
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
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(5),
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
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(6),
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
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(7),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`f32`),
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
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        12,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        21,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        29,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`MnistLabel`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`MnistLabel`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        29,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Zero`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Zero`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    2,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll MnistLabel Zero`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        34,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`MnistLabel`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`MnistLabel`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        34,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Zero`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Zero`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    5,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAllResult MnistLabel Zero`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Ritchie {
                                    ritchie_kind: RitchieKind::Type(
                                        RitchieTypeKind::Item(
                                            RitchieItemKind::Gn,
                                        ),
                                    ),
                                    params: [
                                        FlyRitchieParameter::Variadic(
                                            FlyRitchieVariadicParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                            },
                                        ),
                                        FlyRitchieParameter::Keyed(
                                            FlyRitchieKeyedParameter {
                                                key: `skip`,
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                has_default: true,
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`gn((f32, i32) -> OneVsAllResult MnistLabel Zero`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        51,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        57,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        63,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        89,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`MnistLabel`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`MnistLabel`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        89,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Zero`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Zero`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    12,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    13,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll MnistLabel Zero`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 14,
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
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
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
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec RawContour`),
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
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
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
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
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
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
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
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
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
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
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
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
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
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
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
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec ConnectedComponent`),
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
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
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
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
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
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 23,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 24,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
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
                                    idx: 25,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
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
                                    idx: 26,
                                    src: ExpectationSource {
                                        syn_expr_idx: 24,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
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
                                    idx: 27,
                                    src: ExpectationSource {
                                        syn_expr_idx: 25,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                    idx: 28,
                                    src: ExpectationSource {
                                        syn_expr_idx: 26,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 29,
                                    src: ExpectationSource {
                                        syn_expr_idx: 27,
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
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 30,
                                    src: ExpectationSource {
                                        syn_expr_idx: 28,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAll MnistLabel Zero`),
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
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 32,
                                    src: ExpectationSource {
                                        syn_expr_idx: 29,
                                        kind: Expectation(
                                            31,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 33,
                                    src: ExpectationSource {
                                        syn_expr_idx: 29,
                                        kind: Expectation(
                                            31,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                3,
                                            ),
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
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::AnyOriginal,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 34,
                                    src: ExpectationSource {
                                        syn_expr_idx: 30,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                        ),
                                    },
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
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Pure,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`Leash Vec ConcaveComponent`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Pure,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Leash Vec ConcaveComponent`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 35,
                                    src: ExpectationSource {
                                        syn_expr_idx: 31,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Leashed {
                                                                place_idx: None,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 36,
                                    src: ExpectationSource {
                                        syn_expr_idx: 32,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                    idx: 37,
                                    src: ExpectationSource {
                                        syn_expr_idx: 33,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
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
                                expectation: Expectation::EqsRitchieType(
                                    ExpectEqsRitchieType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 38,
                                    src: ExpectationSource {
                                        syn_expr_idx: 34,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsRitchieCallType(
                                                ExpectEqsRitchieTypeOutcome {
                                                    ritchie_ty_kind: Item(
                                                        Gn,
                                                    ),
                                                    parameter_contracted_tys: [
                                                        Variadic(
                                                            FlyRitchieVariadicParameter {
                                                                contract: Pure,
                                                                ty: FlyTerm {
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
                                                            },
                                                        ),
                                                        Keyed(
                                                            FlyRitchieKeyedParameter {
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 295,
                                                                        },
                                                                    ),
                                                                ),
                                                                contract: Pure,
                                                                ty: FlyTerm {
                                                                    place: None,
                                                                    base: Eth(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 123,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                has_default: true,
                                                            },
                                                        ),
                                                    ],
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: Hol(
                                                            HolTerm(
                                                                7,
                                                            ),
                                                        ),
                                                    },
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
                                        syn_expr_idx: 35,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
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
                                expectation: Expectation::Coersion(
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
                                    idx: 40,
                                    src: ExpectationSource {
                                        syn_expr_idx: 36,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: ImmutableOnStack {
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
                                    idx: 41,
                                    src: ExpectationSource {
                                        syn_expr_idx: 37,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
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
                                expectation: Expectation::Coersion(
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
                                    idx: 42,
                                    src: ExpectationSource {
                                        syn_expr_idx: 38,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: ImmutableOnStack {
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
                                    idx: 43,
                                    src: ExpectationSource {
                                        syn_expr_idx: 39,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
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
                                expectation: Expectation::Coersion(
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
                                    idx: 44,
                                    src: ExpectationSource {
                                        syn_expr_idx: 40,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: ImmutableOnStack {
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 45,
                                    src: ExpectationSource {
                                        syn_expr_idx: 41,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAllResult MnistLabel Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 46,
                                    src: ExpectationSource {
                                        syn_expr_idx: 42,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 47,
                                    src: ExpectationSource {
                                        syn_expr_idx: 42,
                                        kind: Expectation(
                                            46,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                5,
                                            ),
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 48,
                                    src: ExpectationSource {
                                        syn_expr_idx: 42,
                                        kind: Expectation(
                                            46,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                6,
                                            ),
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
                                expectation: Expectation::Coersion(
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
                                    idx: 49,
                                    src: ExpectationSource {
                                        syn_expr_idx: 43,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                    idx: 50,
                                    src: ExpectationSource {
                                        syn_expr_idx: 44,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
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
                                    idx: 51,
                                    src: ExpectationSource {
                                        syn_expr_idx: 45,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 52,
                                    src: ExpectationSource {
                                        syn_expr_idx: 46,
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
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 53,
                                    src: ExpectationSource {
                                        syn_expr_idx: 47,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                    idx: 54,
                                    src: ExpectationSource {
                                        syn_expr_idx: 48,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
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
                                    idx: 55,
                                    src: ExpectationSource {
                                        syn_expr_idx: 49,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`EffHoles`),
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
                                    idx: 56,
                                    src: ExpectationSource {
                                        syn_expr_idx: 50,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash RawContour`),
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
                                    idx: 57,
                                    src: ExpectationSource {
                                        syn_expr_idx: 51,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                9,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 58,
                                    src: ExpectationSource {
                                        syn_expr_idx: 52,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 59,
                                    src: ExpectationSource {
                                        syn_expr_idx: 52,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 60,
                                    src: ExpectationSource {
                                        syn_expr_idx: 53,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                    idx: 61,
                                    src: ExpectationSource {
                                        syn_expr_idx: 54,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
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
                                    idx: 62,
                                    src: ExpectationSource {
                                        syn_expr_idx: 55,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`EffHoles`),
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
                                    idx: 63,
                                    src: ExpectationSource {
                                        syn_expr_idx: 56,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash RawContour`),
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
                                    idx: 64,
                                    src: ExpectationSource {
                                        syn_expr_idx: 57,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                10,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 65,
                                    src: ExpectationSource {
                                        syn_expr_idx: 58,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 66,
                                    src: ExpectationSource {
                                        syn_expr_idx: 58,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 67,
                                    src: ExpectationSource {
                                        syn_expr_idx: 59,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                    idx: 68,
                                    src: ExpectationSource {
                                        syn_expr_idx: 60,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConnectedComponent`),
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
                                    idx: 69,
                                    src: ExpectationSource {
                                        syn_expr_idx: 61,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`EffHoles`),
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
                                    idx: 70,
                                    src: ExpectationSource {
                                        syn_expr_idx: 62,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash RawContour`),
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
                                    idx: 71,
                                    src: ExpectationSource {
                                        syn_expr_idx: 63,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                11,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 72,
                                    src: ExpectationSource {
                                        syn_expr_idx: 64,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                11,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 73,
                                    src: ExpectationSource {
                                        syn_expr_idx: 64,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 74,
                                    src: ExpectationSource {
                                        syn_expr_idx: 65,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 75,
                                    src: ExpectationSource {
                                        syn_expr_idx: 66,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash RawContour`),
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
                                    idx: 76,
                                    src: ExpectationSource {
                                        syn_expr_idx: 67,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
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
                                    idx: 77,
                                    src: ExpectationSource {
                                        syn_expr_idx: 71,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                    idx: 78,
                                    src: ExpectationSource {
                                        syn_expr_idx: 68,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 79,
                                    src: ExpectationSource {
                                        syn_expr_idx: 69,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash RawContour`),
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
                                    idx: 80,
                                    src: ExpectationSource {
                                        syn_expr_idx: 70,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
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
                                expectation: Expectation::Coersion(
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
                                    idx: 81,
                                    src: ExpectationSource {
                                        syn_expr_idx: 72,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                    idx: 82,
                                    src: ExpectationSource {
                                        syn_expr_idx: 73,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                    idx: 83,
                                    src: ExpectationSource {
                                        syn_expr_idx: 74,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash LineSegmentSketch`),
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
                                    idx: 84,
                                    src: ExpectationSource {
                                        syn_expr_idx: 75,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
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
                                    idx: 85,
                                    src: ExpectationSource {
                                        syn_expr_idx: 78,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                    idx: 86,
                                    src: ExpectationSource {
                                        syn_expr_idx: 76,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash LineSegmentSketch`),
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
                                    idx: 87,
                                    src: ExpectationSource {
                                        syn_expr_idx: 77,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
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
                                expectation: Expectation::Coersion(
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
                                    idx: 88,
                                    src: ExpectationSource {
                                        syn_expr_idx: 79,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                    idx: 89,
                                    src: ExpectationSource {
                                        syn_expr_idx: 80,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                    idx: 90,
                                    src: ExpectationSource {
                                        syn_expr_idx: 81,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(4),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 91,
                                    src: ExpectationSource {
                                        syn_expr_idx: 82,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(5),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: ImmutableOnStack {
                                                                place: Idx(
                                                                    PlaceIdx(5),
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
                                    idx: 92,
                                    src: ExpectationSource {
                                        syn_expr_idx: 83,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                    idx: 93,
                                    src: ExpectationSource {
                                        syn_expr_idx: 84,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(6),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 94,
                                    src: ExpectationSource {
                                        syn_expr_idx: 85,
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
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
                                                            expectee_quary: Const,
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
                                    idx: 95,
                                    src: ExpectationSource {
                                        syn_expr_idx: 86,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
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
                                    idx: 96,
                                    src: ExpectationSource {
                                        syn_expr_idx: 87,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
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
                                    idx: 97,
                                    src: ExpectationSource {
                                        syn_expr_idx: 88,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAll MnistLabel Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 98,
                                    src: ExpectationSource {
                                        syn_expr_idx: 89,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                14,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 99,
                                    src: ExpectationSource {
                                        syn_expr_idx: 89,
                                        kind: Expectation(
                                            98,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                12,
                                            ),
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 100,
                                    src: ExpectationSource {
                                        syn_expr_idx: 89,
                                        kind: Expectation(
                                            98,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                13,
                                            ),
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAll MnistLabel Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 101,
                                    src: ExpectationSource {
                                        syn_expr_idx: 90,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                14,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFlyCoersion {
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
                EthTerm(`OneVsAll MnistLabel Zero`),
            ),
            self_ty: None,
        },
    },
]
```