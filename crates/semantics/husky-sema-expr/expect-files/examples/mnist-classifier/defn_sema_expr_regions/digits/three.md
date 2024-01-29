[
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: SynNodeRegionPath::Defn(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
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
                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash Vec ConcaveComponent`),
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
                                            FugitivePath(`mnist_classifier::digits::three::downarc`, `FunctionFn`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
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
                                            FugitivePath(`mnist_classifier::digits::three::uparc`, `FunctionFn`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
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
                                            FugitivePath(`mnist_classifier::digits::three::back`, `FunctionFn`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
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
                                            sema_expr_idx: SemaExprIdx(
                                                3,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        },
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                4,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                        },
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                5,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    element_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Leash Vec ConcaveComponent`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    2,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Leashed,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    6,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
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
                                        12,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
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
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
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
                                        7,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
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
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    8,
                    (
                        SemaExprIdx(
                            8,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: [],
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`FermiMatchResult`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Pure,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: FluffyTermBase::Ethereal(
                                                                            EtherealTerm(`Leash Vec ConcaveComponent`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Pure,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: FluffyTermBase::Ethereal(
                                                                            EtherealTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Leash Vec ConcaveComponent`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Leashed,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`FermiMatchResult`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`FermiMatchResult`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
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
                EtherealTerm(`FermiMatchResult`),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: SynNodeRegionPath::Defn(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash Vec ConcaveComponent`),
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
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            4,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Vec ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EtherealTermSymbol(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 47,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
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
                                        5,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
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
                                        Geq,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ropd: SemaExprIdx(
                                        3,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
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
                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        5,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Vec ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EtherealTermSymbol(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 47,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
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
                                        13,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
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
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            4,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
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
                                        Leq,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
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
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
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
                                            FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
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
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec Option Leash ConcaveComponent`),
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
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
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
                                    owner_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        23,
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
                                        25,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: FluffyIndexSignature::Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
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
                                            FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        13,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        33,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
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
                                    owner_sema_expr_idx: SemaExprIdx(
                                        14,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                15,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: FluffyIndexSignature::Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
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
                                            FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    19,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        17,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            40,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    20,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        42,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    21,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        18,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                19,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: FluffyIndexSignature::Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    23,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    24,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Be {
                                    src: SemaExprIdx(
                                        21,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    target: BePatternSynSyndicate {
                                        pattern_expr_root: BeSynPatternExprRoot {
                                            syn_pattern_expr_idx: 5,
                                        },
                                        variables: ArenaIdxRange(
                                            4..4,
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    25,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    26,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sema_expr_idx: SemaExprIdx(
                                        23,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    27,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        24,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            55,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                                            ),
                                            instantiation: FluffyInstantiation {
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    28,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        57,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 77,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    29,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        25,
                                    ),
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    ropd: SemaExprIdx(
                                        26,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    30,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `uparc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    31,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Be {
                                    src: SemaExprIdx(
                                        28,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        60,
                                    ),
                                    target: BePatternSynSyndicate {
                                        pattern_expr_root: BeSynPatternExprRoot {
                                            syn_pattern_expr_idx: 7,
                                        },
                                        variables: ArenaIdxRange(
                                            4..4,
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    32,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        68,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    33,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sema_expr_idx: SemaExprIdx(
                                        30,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        69,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    34,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        31,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        70,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end_tangent`,
                                        regional_token_idx: RegionalTokenIdx(
                                            71,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        72,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        73,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    35,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    LiteralData::Bool(
                                        True,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    36,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        32,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle`,
                                        regional_token_idx: RegionalTokenIdx(
                                            75,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Vector2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`bool`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        76,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`bool`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    33,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        78,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    37,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `de`,
                                    regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    38,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        82,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 78,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    39,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
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
                                        Greater,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        81,
                                    ),
                                    ropd: SemaExprIdx(
                                        36,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    40,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `de`,
                                    regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    41,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        87,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    42,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                3,
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
                                        86,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        39,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    43,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                3,
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
                                        38,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        85,
                                    ),
                                    ropd: SemaExprIdx(
                                        40,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    44,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        37,
                                    ),
                                    opr: ShortCircuitLogic(
                                        Or,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        83,
                                    ),
                                    ropd: SemaExprIdx(
                                        41,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    45,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    46,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sema_expr_idx: SemaExprIdx(
                                        43,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        92,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    47,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        44,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        93,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            94,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Point2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        95,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    48,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `uparc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        100,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    49,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sema_expr_idx: SemaExprIdx(
                                        46,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    50,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        47,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        102,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            103,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Point2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        104,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    51,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `downarc_enpoint`,
                                    regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    52,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `uparc_startpoint`,
                                    regional_token_idx: RegionalTokenIdx(
                                        113,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    53,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        49,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        110,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `dist`,
                                        regional_token_idx: RegionalTokenIdx(
                                            111,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Point2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FluffyRitchieParameter::Regular(
                                                        FluffyRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: FluffyTermBase::Ethereal(
                                                                    EtherealTerm(`Point2d`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: ImmutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    5,
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
                                        112,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Point2d`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    50,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            6,
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
                                        114,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    54,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `distance`,
                                    regional_token_idx: RegionalTokenIdx(
                                        116,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    7,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    55,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        118,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 80,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    56,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        52,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        117,
                                    ),
                                    ropd: SemaExprIdx(
                                        53,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    57,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
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
                                            FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    58,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        55,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        121,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            122,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                            ),
                                            instantiation: FluffyInstantiation {
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    59,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        124,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 81,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    60,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        56,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        123,
                                    ),
                                    ropd: SemaExprIdx(
                                        57,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    61,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        126,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    62,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sema_expr_idx: SemaExprIdx(
                                        59,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        127,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    63,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        60,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        128,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change`,
                                        regional_token_idx: RegionalTokenIdx(
                                            129,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                                            ),
                                            instantiation: FluffyInstantiation {
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    64,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Literal(
                                    RegionalTokenIdx(
                                        132,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 82,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    65,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                4,
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
                                        131,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        62,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    66,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                4,
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
                                        61,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        130,
                                    ),
                                    ropd: SemaExprIdx(
                                        63,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    67,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 13,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ident: `Yes`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    5,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
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
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    68,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                7,
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
                                            1..18,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    71,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                7,
                                            ),
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
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    condition: Other {
                                        sema_expr_idx: SemaExprIdx(
                                            4,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                                        sema_expr_idx: SemaExprIdx(
                                            8,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
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
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            19,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            26,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            28,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        16,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            35,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 3,
                                        },
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            37,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        20,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            44,
                                        ),
                                    },
                                    condition: Be {
                                        src: SemaExprIdx(
                                            21,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            46,
                                        ),
                                        target: BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 5,
                                            },
                                            variables: ArenaIdxRange(
                                                4..4,
                                            ),
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            51,
                                        ),
                                    },
                                    condition: Other {
                                        sema_expr_idx: SemaExprIdx(
                                            27,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            58,
                                        ),
                                    },
                                    condition: Be {
                                        src: SemaExprIdx(
                                            28,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            60,
                                        ),
                                        target: BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 7,
                                            },
                                            variables: ArenaIdxRange(
                                                4..4,
                                            ),
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            65,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 8,
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
                                            67,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        34,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            79,
                                        ),
                                    },
                                    condition: Other {
                                        sema_expr_idx: SemaExprIdx(
                                            42,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            88,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 9,
                                        },
                                        variables: ArenaIdxRange(
                                            5..6,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            90,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        45,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            97,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 10,
                                        },
                                        variables: ArenaIdxRange(
                                            6..7,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            99,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        48,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            106,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 11,
                                        },
                                        variables: ArenaIdxRange(
                                            7..8,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            108,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        51,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                                        sema_expr_idx: SemaExprIdx(
                                            54,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            119,
                                        ),
                                    },
                                    condition: Other {
                                        sema_expr_idx: SemaExprIdx(
                                            58,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            125,
                                        ),
                                    },
                                    condition: Other {
                                        sema_expr_idx: SemaExprIdx(
                                            64,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        65,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
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
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    66,
                    (
                        SemaExprIdx(
                            66,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: Some(
                                Leashed,
                            ),
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
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
                        FluffyTerm {
                            place: Some(
                                Leashed,
                            ),
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
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
                        FluffyTerm {
                            place: Some(
                                Leashed,
                            ),
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
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
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
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
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
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
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
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
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 30,
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
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 30,
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
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 58,
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 58,
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 58,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    None,
                    None,
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
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
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 30,
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
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 30,
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
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
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
                        3,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    I32(
                                        2,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        7,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    I32(
                                        4,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        11,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
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
                (
                    SemaExprIdx(
                        15,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
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
                (
                    SemaExprIdx(
                        19,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        26,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        33,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        36,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
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
                (
                    SemaExprIdx(
                        39,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 26,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        53,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 16,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        57,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 21,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        62,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 26,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Option Leash ConcaveComponent`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Option Leash ConcaveComponent`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Option Leash ConcaveComponent`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`f32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Point2d`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Point2d`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`f32`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: [],
                    },
                    hollow_terms: HollowTerms {
                        entries: [
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        11,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`usize`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        15,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`usize`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        19,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`usize`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        37,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: Some(
                                                    ImmutableStackOwned {
                                                        location: StackLocationIdx(
                                                            ShiftedU32(
                                                                4,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        61,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: Some(
                                                Leashed,
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: Some(
                                                    Leashed,
                                                ),
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        65,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`MnistLabel`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`MnistLabel`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`MnistLabel`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        65,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Three`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`Three`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    5,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`OneVsAll MnistLabel Three`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 7,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`i32`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`usize`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 23,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 24,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 25,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 26,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 27,
                                    src: ExpectationSource {
                                        syn_expr_idx: 24,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 28,
                                    src: ExpectationSource {
                                        syn_expr_idx: 25,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                Leashed,
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 29,
                                    src: ExpectationSource {
                                        syn_expr_idx: 26,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 30,
                                    src: ExpectationSource {
                                        syn_expr_idx: 27,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 31,
                                    src: ExpectationSource {
                                        syn_expr_idx: 28,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 32,
                                    src: ExpectationSource {
                                        syn_expr_idx: 29,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 33,
                                    src: ExpectationSource {
                                        syn_expr_idx: 30,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 34,
                                    src: ExpectationSource {
                                        syn_expr_idx: 31,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 35,
                                    src: ExpectationSource {
                                        syn_expr_idx: 32,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 36,
                                    src: ExpectationSource {
                                        syn_expr_idx: 33,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 37,
                                    src: ExpectationSource {
                                        syn_expr_idx: 34,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 38,
                                    src: ExpectationSource {
                                        syn_expr_idx: 35,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 39,
                                    src: ExpectationSource {
                                        syn_expr_idx: 36,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`bool`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 41,
                                    src: ExpectationSource {
                                        syn_expr_idx: 38,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 42,
                                    src: ExpectationSource {
                                        syn_expr_idx: 37,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 43,
                                    src: ExpectationSource {
                                        syn_expr_idx: 39,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 44,
                                    src: ExpectationSource {
                                        syn_expr_idx: 41,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 45,
                                    src: ExpectationSource {
                                        syn_expr_idx: 42,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 46,
                                    src: ExpectationSource {
                                        syn_expr_idx: 43,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 47,
                                    src: ExpectationSource {
                                        syn_expr_idx: 44,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 48,
                                    src: ExpectationSource {
                                        syn_expr_idx: 45,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 49,
                                    src: ExpectationSource {
                                        syn_expr_idx: 46,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 50,
                                    src: ExpectationSource {
                                        syn_expr_idx: 47,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 51,
                                    src: ExpectationSource {
                                        syn_expr_idx: 48,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 52,
                                    src: ExpectationSource {
                                        syn_expr_idx: 49,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Point2d`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 53,
                                    src: ExpectationSource {
                                        syn_expr_idx: 50,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: ImmutableStackOwned {
                                                                location: StackLocationIdx(
                                                                    ShiftedU32(
                                                                        6,
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 54,
                                    src: ExpectationSource {
                                        syn_expr_idx: 51,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 55,
                                    src: ExpectationSource {
                                        syn_expr_idx: 52,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            7,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 56,
                                    src: ExpectationSource {
                                        syn_expr_idx: 53,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 57,
                                    src: ExpectationSource {
                                        syn_expr_idx: 54,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 58,
                                    src: ExpectationSource {
                                        syn_expr_idx: 55,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 59,
                                    src: ExpectationSource {
                                        syn_expr_idx: 56,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                Leashed,
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 60,
                                    src: ExpectationSource {
                                        syn_expr_idx: 57,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 61,
                                    src: ExpectationSource {
                                        syn_expr_idx: 58,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 62,
                                    src: ExpectationSource {
                                        syn_expr_idx: 59,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 63,
                                    src: ExpectationSource {
                                        syn_expr_idx: 60,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 64,
                                    src: ExpectationSource {
                                        syn_expr_idx: 62,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 65,
                                    src: ExpectationSource {
                                        syn_expr_idx: 61,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                Leashed,
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 66,
                                    src: ExpectationSource {
                                        syn_expr_idx: 63,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 67,
                                    src: ExpectationSource {
                                        syn_expr_idx: 64,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`OneVsAll MnistLabel Three`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 68,
                                    src: ExpectationSource {
                                        syn_expr_idx: 65,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`MnistLabel`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 69,
                                    src: ExpectationSource {
                                        syn_expr_idx: 65,
                                        kind: Expectation(
                                            68,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                5,
                                            ),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Three`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 70,
                                    src: ExpectationSource {
                                        syn_expr_idx: 65,
                                        kind: Expectation(
                                            68,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                6,
                                            ),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`OneVsAll MnistLabel Three`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 71,
                                    src: ExpectationSource {
                                        syn_expr_idx: 66,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
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
                EtherealTerm(`OneVsAll MnistLabel Three`),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::three::uparc`, `FunctionFn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: SynNodeRegionPath::Defn(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::three::uparc`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
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
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
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
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
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
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 83,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        4,
                                    ),
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `Some`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
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
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                        18,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
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
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
                                            ),
                                            instantiation: FluffyInstantiation {
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`BoundingBox`),
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
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        23,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
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
                                        17,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    11,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
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
                                        25,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
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
                                            1..4,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
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
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                                        sema_expr_idx: SemaExprIdx(
                                            6,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
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
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    13,
                    (
                        SemaExprIdx(
                            13,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
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
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
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
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        5,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
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
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                StackPure {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Vector2d`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: [],
                    },
                    hollow_terms: HollowTerms {
                        entries: [
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        7,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`core::option::Option`, `Enum`),
                                    refined_path: Left(
                                        PreludeTypePath::Option,
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`Option f32`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`fn((move  f32) -> Option f32`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 1,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Hollow(
                                                            HollowTerm(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: FluffyTermBase::Hollow(
                                                                            HollowTerm(
                                                                                0,
                                                                            ),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`BoundingBox`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
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
                EtherealTerm(`Option f32`),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::three::downarc`, `FunctionFn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: SynNodeRegionPath::Defn(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::three::downarc`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
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
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
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
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
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
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        4,
                                    ),
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `Some`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
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
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                        18,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
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
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
                                            ),
                                            instantiation: FluffyInstantiation {
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`BoundingBox`),
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
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        23,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
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
                                        17,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    11,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
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
                                        25,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
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
                                            1..4,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
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
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                                        sema_expr_idx: SemaExprIdx(
                                            6,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
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
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    13,
                    (
                        SemaExprIdx(
                            13,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
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
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
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
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        5,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
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
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                StackPure {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Vector2d`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: [],
                    },
                    hollow_terms: HollowTerms {
                        entries: [
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        7,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`core::option::Option`, `Enum`),
                                    refined_path: Left(
                                        PreludeTypePath::Option,
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`Option f32`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`fn((move  f32) -> Option f32`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 1,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Hollow(
                                                            HollowTerm(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: FluffyTermBase::Hollow(
                                                                            HollowTerm(
                                                                                0,
                                                                            ),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`BoundingBox`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
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
                EtherealTerm(`Option f32`),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::three::back`, `FunctionFn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: SynNodeRegionPath::Defn(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::three::back`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
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
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
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
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
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
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 85,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        4,
                                    ),
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        5,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `Some`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
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
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                        18,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
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
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
                                            ),
                                            instantiation: FluffyInstantiation {
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`BoundingBox`),
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
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
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
                                        23,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
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
                                        17,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    11,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
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
                                        25,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
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
                                            1..4,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
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
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                                        sema_expr_idx: SemaExprIdx(
                                            6,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
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
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    13,
                    (
                        SemaExprIdx(
                            13,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
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
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
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
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        5,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
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
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                StackPure {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Vector2d`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: [],
                    },
                    hollow_terms: HollowTerms {
                        entries: [
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        7,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`core::option::Option`, `Enum`),
                                    refined_path: Left(
                                        PreludeTypePath::Option,
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`Option f32`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`fn((move  f32) -> Option f32`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 1,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vector2d`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`f32`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Hollow(
                                                            HollowTerm(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: FluffyTermBase::Hollow(
                                                                            HollowTerm(
                                                                                0,
                                                                            ),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`BoundingBox`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
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
                EtherealTerm(`Option f32`),
            ),
            self_ty: None,
        },
    },
]