```rust
[
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                            FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Ritchie(
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
                    FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Ritchie(
                            Fn,
                        )`),
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
                                    value: 230,
                                },
                            ),
                        ),
                    },
                ],
                next: 1,
            },
            sem_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
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
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        0,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
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
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
                                SemaExprData::CurrentSynSymbol {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
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
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        2,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
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
                                    3,
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
                                        14,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 96,
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
                                    4,
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
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        4,
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
                                    5,
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
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
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
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        6,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
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
                                SemaExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    opd: SemaExprIdx(
                                        7,
                                    ),
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
                                    8,
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
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            0..3,
                                        ),
                                    ),
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
                                    9,
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
                                            1,
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
                                            3,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        1,
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
                                            9,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            5,
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
                                        8,
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
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    9,
                    (
                        SemaExprIdx(
                            9,
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
                                                    value: 32,
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
                                                            value: 32,
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
                        4,
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
                                EthTerm(`Vector2d`),
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
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
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
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
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
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
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
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
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
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
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
                    FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                            FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Ritchie(
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Ritchie(
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
                                    3,
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
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        },
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                3,
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
                                        9,
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
                                    4,
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
                                                    4,
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
                                        10,
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
                                    6,
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
                                        5,
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
                    6,
                    (
                        SemaExprIdx(
                            6,
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
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
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
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
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
                    FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Ritchie(
                            Fn,
                        )`),
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
                                    value: 230,
                                },
                            ),
                        ),
                    },
                ],
                next: 1,
            },
            sem_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
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
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        0,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
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
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
                                SemaExprData::CurrentSynSymbol {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
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
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        2,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
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
                                    3,
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
                                        14,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 97,
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
                                    4,
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
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        4,
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
                                    5,
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
                                        16,
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
                                    6,
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
                                        6,
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
                                        17,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
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
                                                    EthTerm(`RelativeBoundingBox`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::relative_bounding_box`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::relative_bounding_box`, `MemoizedField`),
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
                                        EthTerm(`RelativeBoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`RelativeBoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
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
                                                    TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::ymax`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`RelativeBoundingBox`),
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
                                                            TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::ymax`, `MethodRitchie(
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
                                        21,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        22,
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
                                    8,
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
                                        24,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 98,
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
                                    9,
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
                                        8,
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
                                        23,
                                    ),
                                    ropd: SemaExprIdx(
                                        9,
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
                                    10,
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
                                        25,
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
                                    11,
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
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        11,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            27,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
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
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::end`, `MethodRitchie(
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
                                                        EthTerm(`Point2d`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::end`, `MethodRitchie(
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
                                        EthTerm(`Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        12,
                                    ),
                                    self_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Transient,
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
                                            Transient,
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
                                            0..4,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        place: Some(
                                            Transient,
                                        ),
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
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
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
                                            3,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        1,
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
                                            9,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            5,
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
                                            15,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            10,
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
                                        13,
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
                                    place: Some(
                                        Transient,
                                    ),
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
                    14,
                    (
                        SemaExprIdx(
                            14,
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
                                                    value: 32,
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
                                                            value: 32,
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
                        4,
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
                        9,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.6f32`),
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
                                EthTerm(`Vector2d`),
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
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
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
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
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
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
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
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`RelativeBoundingBox`),
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
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
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
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
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
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
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
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
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
                                                EthTerm(`Option f32`),
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
                                            Transient,
                                        ),
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
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Transient,
                                        ),
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
                    FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Ritchie(
                            Fn,
                        )`),
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
                                    value: 230,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 1,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 355,
                                },
                            ),
                        ),
                    },
                ],
                next: 2,
            },
            sem_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
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
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        0,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
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
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
                                SemaExprData::CurrentSynSymbol {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
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
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        2,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
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
                                    3,
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
                                        14,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 99,
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
                                    4,
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
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        4,
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
                                    5,
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
                                        16,
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
                                    6,
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
                                        6,
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
                                        17,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
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
                                                    EthTerm(`RelativeBoundingBox`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::relative_bounding_box`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::relative_bounding_box`, `MemoizedField`),
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
                                        EthTerm(`RelativeBoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`RelativeBoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
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
                                                    TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::ymin`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`RelativeBoundingBox`),
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
                                                            TypeItemPath(`<mnist_classifier::geom2d::RelativeBoundingBox(0)>::ymin`, `MethodRitchie(
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
                                        21,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        22,
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
                                    8,
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
                                        24,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 100,
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
                                    9,
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
                                        8,
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
                                        23,
                                    ),
                                    ropd: SemaExprIdx(
                                        9,
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
                                    10,
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
                                        28,
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
                                    11,
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
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        11,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start_tangent`,
                                        regional_token_idx: RegionalTokenIdx(
                                            30,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
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
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::start_tangent`, `MethodRitchie(
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
                                                            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::start_tangent`, `MethodRitchie(
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
                                        31,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        32,
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
                                    12,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        36,
                                    ),
                                    LiteralTokenData::Bool(
                                        True,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle`,
                                        regional_token_idx: RegionalTokenIdx(
                                            34,
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
                                                    TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::angle`, `MethodRitchie(
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
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Simple(
                                                        FlyRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: FlyTerm {
                                                                place: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`bool`),
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
                                                            TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::angle`, `MethodRitchie(
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
                                        35,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`bool`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemaExprIdx(
                                                    13,
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
                                SemaExprData::CurrentSynSymbol {
                                    ident: `ang`,
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 1,
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
                                    15,
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
                                        41,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 101,
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
                                    16,
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
                                        15,
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
                                        40,
                                    ),
                                    ropd: SemaExprIdx(
                                        16,
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
                                    17,
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
                                    ident: `ang`,
                                    regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 1,
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
                                    18,
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
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            0..6,
                                        ),
                                    ),
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
                                    19,
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
                                            1,
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
                                            3,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        1,
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
                                            9,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            5,
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
                                            15,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            10,
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
                                            25,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 1,
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
                                            27,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            38,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            17,
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
                                        18,
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
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    19,
                    (
                        SemaExprIdx(
                            19,
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
                                                    value: 32,
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
                                                            value: 32,
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
                ],
            },
            sem_expr_terms: [
                (
                    SemaExprIdx(
                        4,
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
                        9,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.3f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        13,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`true`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        16,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`30.0f32`),
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
                                EthTerm(`Vector2d`),
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
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
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
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
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
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
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
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`RelativeBoundingBox`),
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
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
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
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
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
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
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
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`bool`),
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
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
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
                                        syn_expr_idx: 14,
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
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
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
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
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
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
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
                                                EthTerm(`Option f32`),
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
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
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
                    FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Defn(
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                                    value: 357,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 1,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 358,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 2,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 260,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 3,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 86,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 4,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 358,
                                },
                            ),
                        ),
                    },
                    Variable {
                        current_syn_symbol_idx: 5,
                        ident: Ident(
                            Coword(
                                Id {
                                    value: 277,
                                },
                            ),
                        ),
                    },
                ],
                next: 6,
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
                                        ident: `max_hole_ilen`,
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
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
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
                                                    value: 102,
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
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
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
                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                    4,
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
                                        4,
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
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
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
                                    5,
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
                                    ident: `simple_match_norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        14,
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
                                    6,
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
                                        16,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 103,
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
                                    7,
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
                                        6,
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
                                        15,
                                    ),
                                    ropd: SemaExprIdx(
                                        7,
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
                                    8,
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
                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                    9,
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
                                        9,
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
                                        20,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            21,
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
                                    10,
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
                                        23,
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
                                    11,
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
                                        10,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                11,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        24,
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
                                    13,
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
                                        12,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        25,
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
                                    14,
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
                                    15,
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
                                        14,
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
                                        34,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `upper_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            35,
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
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
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
                                    16,
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
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
                                    17,
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
                                        16,
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
                                        38,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `lower_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            39,
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
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
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
                                    18,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        15,
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
                                        36,
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
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    19,
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
                                    ident: `upper_excess`,
                                    regional_token_idx: RegionalTokenIdx(
                                        41,
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
                                    20,
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
                                        43,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 104,
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
                                    21,
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
                                        19,
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
                                        42,
                                    ),
                                    ropd: SemaExprIdx(
                                        20,
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
                                    22,
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
                                    path_expr_idx: 6,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                    23,
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
                                        22,
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
                                        49,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            50,
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
                                    24,
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
                                        52,
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
                                    25,
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
                                        23,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                24,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        53,
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
                                    27,
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
                                        25,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        54,
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
                                    28,
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
                                        26,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end_tangent`,
                                        regional_token_idx: RegionalTokenIdx(
                                            56,
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
                                                    TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::end_tangent`, `MethodRitchie(
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
                                                            TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::end_tangent`, `MethodRitchie(
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
                                        57,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        58,
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
                                    29,
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
                                SemaExprData::CurrentSynSymbol {
                                    ident: `end_tangent`,
                                    regional_token_idx: RegionalTokenIdx(
                                        62,
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
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    30,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    self_argument: SemaExprIdx(
                                        28,
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
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        63,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            64,
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
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
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
                                    31,
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
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 4,
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
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    32,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
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
                                        69,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 105,
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
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    33,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
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
                                        68,
                                    ),
                                    opd: SemaExprIdx(
                                        31,
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
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    34,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
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
                                        30,
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
                                        67,
                                    ),
                                    ropd: SemaExprIdx(
                                        32,
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
                                    35,
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
                                    path_expr_idx: 8,
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
                                                                    3,
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
                                                                    4,
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
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    36,
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
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `simple_match_norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        75,
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
                                    39,
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
                                        77,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 106,
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
                                    40,
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
                                        35,
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
                                        76,
                                    ),
                                    ropd: SemaExprIdx(
                                        36,
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
                                    41,
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
                                    path_expr_idx: 9,
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
                                    42,
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
                                        38,
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
                                        83,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `upper_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            84,
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
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
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
                                    43,
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 10,
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
                                    44,
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
                                        40,
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
                                        87,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `lower_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            88,
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
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
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
                                    45,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        39,
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
                                        85,
                                    ),
                                    ropd: SemaExprIdx(
                                        41,
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
                                    46,
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
                                    ident: `upper_excess`,
                                    regional_token_idx: RegionalTokenIdx(
                                        90,
                                    ),
                                    current_syn_symbol_idx: 4,
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
                                    47,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        92,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 107,
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
                                    48,
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
                                        43,
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
                                        91,
                                    ),
                                    ropd: SemaExprIdx(
                                        44,
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
                                    49,
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
                                    path_expr_idx: 12,
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
                                                                    6,
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
                                                                    7,
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
                                    50,
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 13,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                    53,
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
                                        47,
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
                                        99,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            100,
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
                                    54,
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
                                        102,
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
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    55,
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
                                        48,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemaExprIdx(
                                                49,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        103,
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
                                    57,
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
                                        50,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        104,
                                    ),
                                    target: BePatternSyndicate {
                                        pattern_expr_root: BeSynPatternExprRoot {
                                            syn_pattern_expr_idx: 8,
                                        },
                                        variables: ArenaIdxRange(
                                            5..5,
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
                                    58,
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
                                            FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                    59,
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
                                        52,
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
                                        113,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            114,
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
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    60,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
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
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        116,
                                    ),
                                    LiteralTokenData::Bool(
                                        False,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    61,
                                    FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
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
                                    path_expr_idx: 17,
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
                                                                    10,
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
                                                                    11,
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
                                            12,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    62,
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
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            10..18,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    65,
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
                                            45,
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
                                            47,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        27,
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
                                            59,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 5,
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
                                            61,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        29,
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
                                            65,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            33,
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
                                            18,
                                        ),
                                    },
                                    condition: Be {
                                        src: SemaExprIdx(
                                            12,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            25,
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
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            30,
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
                                            32,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        18,
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
                                SemaStmtData::IfElse {
                                    if_branch: SemaIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                40,
                                            ),
                                        },
                                        condition: Other {
                                            sem_expr_idx: SemaExprIdx(
                                                21,
                                            ),
                                            conversion: None,
                                        },
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                44,
                                            ),
                                        },
                                        stmts: SemaStmtIdxRange(
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
                                            70,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        34,
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
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            79,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 6,
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
                                            81,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        42,
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
                                            89,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            45,
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
                                            93,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        46,
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            3,
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
                                            7,
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
                                            9,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        5,
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
                                SemaStmtData::IfElse {
                                    if_branch: SemaIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                        },
                                        condition: Other {
                                            sem_expr_idx: SemaExprIdx(
                                                8,
                                            ),
                                            conversion: None,
                                        },
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                17,
                                            ),
                                        },
                                        stmts: SemaStmtIdxRange(
                                            ArenaIdxRange(
                                                3..7,
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
                                SemaStmtData::IfElse {
                                    if_branch: SemaIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                74,
                                            ),
                                        },
                                        condition: Other {
                                            sem_expr_idx: SemaExprIdx(
                                                37,
                                            ),
                                            conversion: None,
                                        },
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                78,
                                            ),
                                        },
                                        stmts: SemaStmtIdxRange(
                                            ArenaIdxRange(
                                                7..10,
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            97,
                                        ),
                                    },
                                    condition: Be {
                                        src: SemaExprIdx(
                                            50,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            104,
                                        ),
                                        target: BePatternSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 8,
                                            },
                                            variables: ArenaIdxRange(
                                                5..5,
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
                                            109,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_expr_idx: 9,
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
                                            111,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemaExprIdx(
                                        53,
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
                                            115,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemaExprIdx(
                                            54,
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
                                        55,
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
                                            12,
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
                    56,
                    (
                        SemaExprIdx(
                            56,
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
                                                    value: 32,
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
                            place: Some(
                                Leashed {
                                    place_idx: None,
                                },
                            ),
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
                                                            value: 32,
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
                                                    value: 60,
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
                        7,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        11,
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
                        20,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`10.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        24,
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
                        31,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`7.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        36,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`4.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        44,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`10.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        49,
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
                        54,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`false`),
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
                                EthTerm(`Vector2d`),
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
                                EthTerm(`f32`),
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
                                Leashed {
                                    place_idx: Some(
                                        PlaceIdx(5),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Vec Leash ConcaveComponent`),
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
                                        11,
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
                                        24,
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
                                        30,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: Some(
                                                    ImmutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                ),
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
                                                EthTerm(`Seven`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Seven`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Seven`),
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
                                                    3,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll MnistLabel Seven`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        46,
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
                                        46,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Seven`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Seven`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Seven`),
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
                                                    6,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll MnistLabel Seven`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        49,
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
                                        55,
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
                                        55,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Seven`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Seven`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Seven`),
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
                                                    10,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    11,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll MnistLabel Seven`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 12,
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
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
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
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
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
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
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
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
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
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
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
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
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
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
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
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
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
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
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
                                    idx: 17,
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
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
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
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
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
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
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
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
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
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
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
                                    idx: 23,
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
                                    idx: 24,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
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
                                    idx: 25,
                                    src: ExpectationSource {
                                        syn_expr_idx: 24,
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
                                    idx: 26,
                                    src: ExpectationSource {
                                        syn_expr_idx: 25,
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
                                    idx: 27,
                                    src: ExpectationSource {
                                        syn_expr_idx: 25,
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
                                    idx: 28,
                                    src: ExpectationSource {
                                        syn_expr_idx: 26,
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
                                    idx: 29,
                                    src: ExpectationSource {
                                        syn_expr_idx: 27,
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
                                    idx: 30,
                                    src: ExpectationSource {
                                        syn_expr_idx: 28,
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
                                    idx: 31,
                                    src: ExpectationSource {
                                        syn_expr_idx: 29,
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
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
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
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 33,
                                    src: ExpectationSource {
                                        syn_expr_idx: 30,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
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
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
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
                                    idx: 34,
                                    src: ExpectationSource {
                                        syn_expr_idx: 32,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
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
                                    idx: 35,
                                    src: ExpectationSource {
                                        syn_expr_idx: 33,
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
                                                EthTerm(`OneVsAll MnistLabel Seven`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 36,
                                    src: ExpectationSource {
                                        syn_expr_idx: 34,
                                        kind: Expr,
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
                                    idx: 37,
                                    src: ExpectationSource {
                                        syn_expr_idx: 34,
                                        kind: Expectation(
                                            36,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Seven`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 38,
                                    src: ExpectationSource {
                                        syn_expr_idx: 34,
                                        kind: Expectation(
                                            36,
                                        ),
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
                                    idx: 39,
                                    src: ExpectationSource {
                                        syn_expr_idx: 35,
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
                                    idx: 40,
                                    src: ExpectationSource {
                                        syn_expr_idx: 36,
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
                                    idx: 41,
                                    src: ExpectationSource {
                                        syn_expr_idx: 37,
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
                                    idx: 42,
                                    src: ExpectationSource {
                                        syn_expr_idx: 38,
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
                                    idx: 43,
                                    src: ExpectationSource {
                                        syn_expr_idx: 40,
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
                                    idx: 44,
                                    src: ExpectationSource {
                                        syn_expr_idx: 39,
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
                                    idx: 45,
                                    src: ExpectationSource {
                                        syn_expr_idx: 41,
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
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 46,
                                    src: ExpectationSource {
                                        syn_expr_idx: 42,
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
                                    idx: 47,
                                    src: ExpectationSource {
                                        syn_expr_idx: 43,
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
                                    idx: 48,
                                    src: ExpectationSource {
                                        syn_expr_idx: 44,
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
                                    idx: 49,
                                    src: ExpectationSource {
                                        syn_expr_idx: 45,
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
                                                EthTerm(`OneVsAll MnistLabel Seven`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 50,
                                    src: ExpectationSource {
                                        syn_expr_idx: 46,
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
                                    idx: 51,
                                    src: ExpectationSource {
                                        syn_expr_idx: 46,
                                        kind: Expectation(
                                            50,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Seven`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 52,
                                    src: ExpectationSource {
                                        syn_expr_idx: 46,
                                        kind: Expectation(
                                            50,
                                        ),
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
                                    idx: 53,
                                    src: ExpectationSource {
                                        syn_expr_idx: 47,
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
                                    idx: 55,
                                    src: ExpectationSource {
                                        syn_expr_idx: 49,
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
                                    idx: 56,
                                    src: ExpectationSource {
                                        syn_expr_idx: 50,
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
                                    idx: 57,
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
                                    idx: 58,
                                    src: ExpectationSource {
                                        syn_expr_idx: 51,
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
                                    idx: 60,
                                    src: ExpectationSource {
                                        syn_expr_idx: 53,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Leash ConcaveComponent`),
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
                                    idx: 61,
                                    src: ExpectationSource {
                                        syn_expr_idx: 54,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Const,
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
                                expectation: Expectation::Coersion(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAll MnistLabel Seven`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 62,
                                    src: ExpectationSource {
                                        syn_expr_idx: 55,
                                        kind: Expr,
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
                                    idx: 63,
                                    src: ExpectationSource {
                                        syn_expr_idx: 55,
                                        kind: Expectation(
                                            62,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                10,
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
                                                EthTerm(`Seven`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 64,
                                    src: ExpectationSource {
                                        syn_expr_idx: 55,
                                        kind: Expectation(
                                            62,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                11,
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
                                                EthTerm(`OneVsAll MnistLabel Seven`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 65,
                                    src: ExpectationSource {
                                        syn_expr_idx: 56,
                                        kind: Expr,
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
                EthTerm(`OneVsAll MnistLabel Seven`),
            ),
            self_ty: None,
        },
    },
]
```