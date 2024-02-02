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
                                            path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                                path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                        EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
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
                                        EthTerm(`Leash Vec ConcaveComponent`),
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
                                            EthTerm(`Leash Vec ConcaveComponent`),
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
                                            FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
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
                                        EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                            sema_expr_idx: SemaExprIdx(
                                                3,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    element_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
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
                                                        EthTerm(`Leash Vec ConcaveComponent`),
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
                                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    4,
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
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`FermiMatchResult`),
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
                                        5,
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
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    6,
                    (
                        SemaExprIdx(
                            6,
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
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
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
                                                            EthTerm(`FermiMatchResult`),
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
                                                                            EthTerm(`Leash Vec ConcaveComponent`),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`Leash Vec ConcaveComponent`),
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
                                            EthTerm(`Leash Vec ConcaveComponent`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`FermiMatchResult`),
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`FermiMatchResult`),
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
                EthTerm(`FermiMatchResult`),
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
                                            path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
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
                                                path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
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
                                        2,
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
                                        EthTerm(`Leash ConcaveComponent`),
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
                                        1,
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
                                                    EthTerm(`f32`),
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
                                        EthTerm(`f32`),
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
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
                                        2,
                                    ),
                                    opr: Closed(
                                        Add,
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
                                        5,
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
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                    5,
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
                                SemaExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        5,
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
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
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
                                SemaExprData::Binary {
                                    lopd: SemaExprIdx(
                                        4,
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
                                        7,
                                    ),
                                    ropd: SemaExprIdx(
                                        6,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                        EthTerm(`Leash ConcaveComponent`),
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
                                                    EthTerm(`f32`),
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
                                        EthTerm(`f32`),
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
                                    opd_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
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
                                        base: FluffyTermBase::Ethereal(
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
                                        10,
                                    ),
                                    opr: Closed(
                                        Add,
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
                                        14,
                                    ),
                                    ropd: SemaExprIdx(
                                        11,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                            1..3,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                            7,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`unit`),
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
                                                coersion: WrapInSome,
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
                                        EthTerm(`f32`),
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
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
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
                                                value: 28,
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
                        11,
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
                                        4,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
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
                                            EthTerm(`Leash ConcaveComponent`),
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
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                                EthTerm(`f32`),
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
                                            EthTerm(`f32`),
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
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
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
                                                EthTerm(`f32`),
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
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
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
                                            EthTerm(`bool`),
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
                                            EthTerm(`Leash ConcaveComponent`),
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
                                            EthTerm(`f32`),
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
                                            EthTerm(`f32`),
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
                                                EthTerm(`f32`),
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
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
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
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
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
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                                                path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                        EthTerm(`Leash ConnectedComponent`),
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
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`Vec RawContour`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`, `MemoizedField`),
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
                                        EthTerm(`Vec RawContour`),
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
                                            EthTerm(`Vec RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        2,
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
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
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
                                                            EthTerm(`Vec RawContour`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            SymbolEthTerm(
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
                                                                                            value: 15,
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
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        3,
                                    ),
                                    opr: Comparison(
                                        Eq,
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
                                        9,
                                    ),
                                    ropd: SemaExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        6,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                                    EthTerm(`f32`),
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
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFluffySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    ropd: SemaExprIdx(
                                        9,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        11,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                                    EthTerm(`Vec Option Leash ConcaveComponent`),
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
                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    13,
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
                                        12,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        26,
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
                                        28,
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
                                                    EthTerm(`Option Leash ConcaveComponent`),
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
                                        EthTerm(`Option Leash ConcaveComponent`),
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
                                        14,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    target: BePatternSynSyndicate {
                                        pattern_expr_root: BeSynPatternExprRoot {
                                            syn_pattern_expr_idx: 3,
                                        },
                                        variables: ArenaIdxRange(
                                            2..2,
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
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
                                        EthTerm(`Vec ConnectedComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec ConnectedComponent`),
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
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
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
                                                            EthTerm(`Vec ConnectedComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            SymbolEthTerm(
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
                                                                                            value: 12,
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
                                        38,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    19,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        17,
                                    ),
                                    opr: Comparison(
                                        Eq,
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
                                        40,
                                    ),
                                    ropd: SemaExprIdx(
                                        18,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    20,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    21,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        20,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                                    EthTerm(`Vec Option Leash ConcaveComponent`),
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
                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    22,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    23,
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
                                        21,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                22,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        50,
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
                                                    EthTerm(`Option Leash ConcaveComponent`),
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
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    25,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Option Leash ConcaveComponent`),
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
                                        51,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    26,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        24,
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
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EthTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`Vector2d`),
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
                                        54,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    27,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        25,
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
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EthTerm(`Vector2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`f32`),
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
                                        58,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    28,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
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
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    29,
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    30,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        27,
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
                                        62,
                                    ),
                                    ropd: SemaExprIdx(
                                        28,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    31,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                                    SymbolEthTerm(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    SymbolEthTerm(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
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
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    32,
                                    FluffyTerm {
                                        place: None,
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 9,
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
                                        EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    35,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
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
                                        EthTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    36,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    element_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    37,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        31,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`Leash Vec ConcaveComponent`),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    32,
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
                                                        74,
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
                                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
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
                                        77,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    38,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`FermiMatchResult`),
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
                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    SymbolEthTerm(
                                                        Id {
                                                            value: 1,
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
                                                    SymbolEthTerm(
                                                        Id {
                                                            value: 2,
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
                                            8,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    39,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
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
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
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
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    40,
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
                                        36,
                                    ),
                                    owner_ty: FluffyTerm {
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`f32`),
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
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
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
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
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
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    42,
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
                                        38,
                                    ),
                                    owner_ty: FluffyTerm {
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
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
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    43,
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
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
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
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    44,
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
                                        40,
                                    ),
                                    owner_ty: FluffyTerm {
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
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
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    45,
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    46,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        35,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieParameterArgumentMatch::Variadic(
                                            FluffyRitchieVariadicParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                            },
                                            [
                                                SemaVariadicCallListItem {
                                                    argument_sema_expr_idx: SemaExprIdx(
                                                        37,
                                                    ),
                                                    coersion_outcome: Some(
                                                        ExpectCoersionOutcome {
                                                            coersion: Trivial(
                                                                TrivialFluffyCoersion {
                                                                    expectee_place: ImmutableStackOwned {
                                                                        location: StackLocationIdx(
                                                                            ShiftedU32(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    separator: Comma(
                                                        RegionalTokenIdx(
                                                            83,
                                                        ),
                                                    ),
                                                },
                                                SemaVariadicCallListItem {
                                                    argument_sema_expr_idx: SemaExprIdx(
                                                        39,
                                                    ),
                                                    coersion_outcome: Some(
                                                        ExpectCoersionOutcome {
                                                            coersion: Trivial(
                                                                TrivialFluffyCoersion {
                                                                    expectee_place: ImmutableStackOwned {
                                                                        location: StackLocationIdx(
                                                                            ShiftedU32(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    separator: Comma(
                                                        RegionalTokenIdx(
                                                            87,
                                                        ),
                                                    ),
                                                },
                                                SemaVariadicCallListItem {
                                                    argument_sema_expr_idx: SemaExprIdx(
                                                        41,
                                                    ),
                                                    coersion_outcome: Some(
                                                        ExpectCoersionOutcome {
                                                            coersion: Trivial(
                                                                TrivialFluffyCoersion {
                                                                    expectee_place: ImmutableStackOwned {
                                                                        location: StackLocationIdx(
                                                                            ShiftedU32(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    separator: Comma(
                                                        RegionalTokenIdx(
                                                            91,
                                                        ),
                                                    ),
                                                },
                                            ],
                                        ),
                                        SemaRitchieParameterArgumentMatch::Keyed(
                                            FluffyRitchieKeyedParameter {
                                                key: `skip`,
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
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
                                                    key: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 295,
                                                            },
                                                        ),
                                                    ),
                                                    argument_sema_expr_idx: SemaExprIdx(
                                                        42,
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
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        95,
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
                                    47,
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
                                SemaExprData::Unveil {
                                    opd_sema_expr_idx: SemaExprIdx(
                                        43,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                    unveil_output_ty_signature: TraitForTypeAssociatedTypeEtherealSignature {
                                        path: TraitForTypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TraitForTypeItem(
                                                        TraitForTypeItemPathData {
                                                            impl_block: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `Output`,
                                                            item_kind: AssociatedType,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        instantiation: EtherealInstantiation {
                                            symbol_map: [
                                                (
                                                    SymbolEthTerm {
                                                        toolchain: Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: VirtualPath {
                                                                    _data: VirtualPathBuf(
                                                                        "../../../library",
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                        ty: EthTerm(`Type`),
                                                        index: EthTermSymbolIndex(
                                                            EthTermSymbolIndexImpl::Type {
                                                                attrs: EthTemplateSymbolAttrs {
                                                                    class: Phantom,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                    },
                                                    EthTerm(`MnistLabel`),
                                                ),
                                                (
                                                    SymbolEthTerm {
                                                        toolchain: Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: VirtualPath {
                                                                    _data: VirtualPathBuf(
                                                                        "../../../library",
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                        ty: EthTerm(`t`),
                                                        index: EthTermSymbolIndex(
                                                            EthTermSymbolIndexImpl::ConstOther {
                                                                attrs: EthTemplateSymbolAttrs {
                                                                    class: Phantom,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                    },
                                                    EthTerm(`Zero`),
                                                ),
                                            ],
                                            separator: Some(
                                                2,
                                            ),
                                        },
                                        ty_term: EthTerm(`unit`),
                                    },
                                    unveil_associated_fn_path: TraitForTypeItemPath(
                                        ItemPathId {
                                            data: ItemPathData::AssociatedItem(
                                                AssociatedItemPathData::TraitForTypeItem(
                                                    TraitForTypeItemPathData {
                                                        impl_block: TraitForTypeImplBlock {
                                                            data: TraitForTypeImplBlockPathData {
                                                                module_path: `malamute`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                        ident: `unveil`,
                                                        item_kind: AssociatedFunctionFn,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    return_ty: EthTerm(`OneVsAll MnistLabel Zero`),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    50,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
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
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    51,
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
                                        45,
                                    ),
                                    owner_ty: FluffyTerm {
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`f32`),
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
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
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
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    53,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        46,
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
                                        101,
                                    ),
                                    ropd: SemaExprIdx(
                                        47,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    54,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                        EthTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    55,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        49,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`EffHoles`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                                        EthTerm(`EffHoles`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    56,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`EffHoles`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        50,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`Vec Option Leash RawContour`),
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
                                        EthTerm(`Vec Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    57,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    58,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
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
                                    owner_sema_expr_idx: SemaExprIdx(
                                        51,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                52,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        111,
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
                                                    EthTerm(`Option Leash RawContour`),
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
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    60,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        53,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        112,
                                    ),
                                    target: BePatternSynSyndicate {
                                        pattern_expr_root: BeSynPatternExprRoot {
                                            syn_pattern_expr_idx: 6,
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
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    61,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 14,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                        EthTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    62,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash ConnectedComponent`),
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`EffHoles`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                                        EthTerm(`EffHoles`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    63,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`EffHoles`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        56,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`Vec Option Leash RawContour`),
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
                                        EthTerm(`Vec Option Leash RawContour`),
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            10,
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
                                    owner_sema_expr_idx: SemaExprIdx(
                                        57,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        120,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                58,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        122,
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
                                                    EthTerm(`Option Leash RawContour`),
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
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    67,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        59,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        123,
                                    ),
                                    target: BePatternSynSyndicate {
                                        pattern_expr_root: BeSynPatternExprRoot {
                                            syn_pattern_expr_idx: 8,
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
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    68,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                        EthTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    69,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash ConnectedComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        61,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`EffHoles`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                                        EthTerm(`EffHoles`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    70,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`EffHoles`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        62,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`Vec Option Leash RawContour`),
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
                                        EthTerm(`Vec Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    71,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            11,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    72,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
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
                                    owner_sema_expr_idx: SemaExprIdx(
                                        63,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        136,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                64,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        138,
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
                                                    EthTerm(`Option Leash RawContour`),
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
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    74,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    75,
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
                                            EthTerm(`Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sema_expr_idx: SemaExprIdx(
                                        66,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        143,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    76,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        67,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
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
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    77,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        68,
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
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EthTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`f32`),
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
                                        148,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        149,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    78,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                        EthTerm(`Option Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    79,
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
                                            EthTerm(`Option Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unwrap {
                                    opd_sema_expr_idx: SemaExprIdx(
                                        70,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        152,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`Leash RawContour`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    80,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash RawContour`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        71,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
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
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    81,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        72,
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
                                                            EthTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`f32`),
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
                                        157,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        158,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    82,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                        69,
                                    ),
                                    opr: Closed(
                                        Sub,
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
                                        150,
                                    ),
                                    ropd: SemaExprIdx(
                                        73,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    83,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                        EthTerm(`Leash LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    84,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        75,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`, `MemoizedField`),
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
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    85,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        76,
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
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFluffySignature::MethodFn(
                                            MethodFnFluffySignature {
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EthTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`f32`),
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
                                        167,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        168,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    86,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 18,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
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
                                        EthTerm(`Leash LineSegmentSketch`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    87,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash LineSegmentSketch`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        78,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`, `MemoizedField`),
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
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    88,
                                    FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        79,
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
                                                            EthTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`f32`),
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
                                        175,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        176,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    89,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                        77,
                                    ),
                                    opr: Closed(
                                        Sub,
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
                                        169,
                                    ),
                                    ropd: SemaExprIdx(
                                        80,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    90,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    91,
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
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    92,
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
                                        82,
                                    ),
                                    opr: Closed(
                                        Div,
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
                                        181,
                                    ),
                                    ropd: SemaExprIdx(
                                        83,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    93,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    94,
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
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    95,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
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
                                        85,
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
                                        185,
                                    ),
                                    ropd: SemaExprIdx(
                                        86,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    96,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
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
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
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
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    97,
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
                                        88,
                                    ),
                                    owner_ty: FluffyTerm {
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
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FluffyFieldSignature::Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`f32`),
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
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    98,
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
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 20,
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
                                                    SymbolEthTerm(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    12,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    SymbolEthTerm(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
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
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            14,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    99,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
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
                                            8..21,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            14,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    102,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
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
            sema_stmt_arena: SemaStmtArena(
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
                                            14,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                        sema_expr_idx: SemaExprIdx(
                                            10,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                            14,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            29,
                                        ),
                                        target: BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 3,
                                            },
                                            variables: ArenaIdxRange(
                                                2..2,
                                            ),
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                        sema_expr_idx: SemaExprIdx(
                                            19,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 4,
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
                                            44,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        26,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                        sema_expr_idx: SemaExprIdx(
                                            29,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                        30,
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
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                            sema_expr_idx: SemaExprIdx(
                                                5,
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
                                                1..8,
                                            ),
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 5,
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
                                            70,
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
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        44,
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
                                        sema_expr_idx: SemaExprIdx(
                                            48,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                            53,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            112,
                                        ),
                                        target: BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 6,
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
                                            59,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            123,
                                        ),
                                        target: BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 8,
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
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 9,
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
                                            130,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        65,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 10,
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
                                            141,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        74,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 11,
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
                                            161,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        81,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 12,
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
                                            179,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        84,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                        sema_expr_idx: SemaExprIdx(
                                            87,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
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
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 13,
                                        },
                                        variables: ArenaIdxRange(
                                            8..9,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            189,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        89,
                                    ),
                                    coersion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        90,
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
                                            14,
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
                    91,
                    (
                        SemaExprIdx(
                            91,
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
                            place: Some(
                                Leashed,
                            ),
                            base: Ethereal(
                                Application(
                                    ApplicationEthTerm(
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
                        FluffyTerm {
                            place: Some(
                                Leashed,
                            ),
                            base: Ethereal(
                                Application(
                                    ApplicationEthTerm(
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
                        FluffyTerm {
                            place: Some(
                                Leashed,
                            ),
                            base: Ethereal(
                                Application(
                                    ApplicationEthTerm(
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
                        FluffyTerm {
                            place: Some(
                                Leashed,
                            ),
                            base: Ethereal(
                                Application(
                                    ApplicationEthTerm(
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
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            3,
                                        ),
                                    ),
                                },
                            ),
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
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            ApplicationEthTerm(
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
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
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
                        4,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        9,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 29,
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
                        13,
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
                        18,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        22,
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
                        28,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 30,
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
                        42,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    I32(
                                        5,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        47,
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
                        52,
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
                        58,
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
                        64,
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
                        86,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
                                            Id {
                                                value: 9,
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
                                EthTerm(`f32`),
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
                                EthTerm(`f32`),
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
                                EthTerm(`FermiMatchResult`),
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
                                EthTerm(`Option Leash RawContour`),
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
                                EthTerm(`f32`),
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
                                EthTerm(`f32`),
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
                                EthTerm(`f32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                ImmutableStackOwned {
                                    location: StackLocationIdx(
                                        ShiftedU32(
                                            8,
                                        ),
                                    ),
                                },
                            ),
                            base: FluffyTermBase::Ethereal(
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
                                        13,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        22,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        30,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`MnistLabel`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`MnistLabel`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        30,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Zero`),
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
                                                    2,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll MnistLabel Zero`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        35,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`MnistLabel`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`MnistLabel`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        35,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Zero`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                    EthTerm(`OneVsAllResult MnistLabel Zero`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Gn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Variadic(
                                            FluffyRitchieVariadicParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                            },
                                        ),
                                        FluffyRitchieParameter::Keyed(
                                            FluffyRitchieKeyedParameter {
                                                key: `skip`,
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                has_default: true,
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`gn((f32, i32) -> OneVsAllResult MnistLabel Zero`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        52,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        58,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        64,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        90,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EthTerm(`MnistLabel`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`MnistLabel`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        90,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Zero`),
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
                                                    12,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    13,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
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
                                            EthTerm(`Leash ConnectedComponent`),
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
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec RawContour`),
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
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`i32`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`i32`),
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`FermiMatchResult`),
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
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
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
                                            EthTerm(`f32`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`FermiMatchResult`),
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
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
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
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
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
                                                EthTerm(`usize`),
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
                                            EthTerm(`Option Leash ConcaveComponent`),
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
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec ConnectedComponent`),
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
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`i32`),
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
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`i32`),
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
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`FermiMatchResult`),
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
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
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
                                    idx: 23,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
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
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 24,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
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
                                    idx: 25,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Option Leash ConcaveComponent`),
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
                                    idx: 26,
                                    src: ExpectationSource {
                                        syn_expr_idx: 24,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash ConcaveComponent`),
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
                                        syn_expr_idx: 25,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vector2d`),
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
                                        syn_expr_idx: 26,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 29,
                                    src: ExpectationSource {
                                        syn_expr_idx: 27,
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
                                            EthTerm(`f32`),
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
                                                            2,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 30,
                                    src: ExpectationSource {
                                        syn_expr_idx: 28,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 31,
                                    src: ExpectationSource {
                                        syn_expr_idx: 29,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                                EthTerm(`OneVsAll MnistLabel Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 32,
                                    src: ExpectationSource {
                                        syn_expr_idx: 30,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
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
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 33,
                                    src: ExpectationSource {
                                        syn_expr_idx: 30,
                                        kind: Expectation(
                                            32,
                                        ),
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
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 34,
                                    src: ExpectationSource {
                                        syn_expr_idx: 30,
                                        kind: Expectation(
                                            32,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                3,
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
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::AnyOriginal,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 35,
                                    src: ExpectationSource {
                                        syn_expr_idx: 31,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
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
                                                            EthTerm(`FermiMatchResult`),
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
                                                                            EthTerm(`Leash Vec ConcaveComponent`),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`Leash Vec ConcaveComponent`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash Vec ConcaveComponent`),
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
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 37,
                                    src: ExpectationSource {
                                        syn_expr_idx: 33,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
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
                                    idx: 38,
                                    src: ExpectationSource {
                                        syn_expr_idx: 34,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsRitchieType(
                                    ExpectEqsRitchieType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 39,
                                    src: ExpectationSource {
                                        syn_expr_idx: 35,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsRitchieCallType(
                                                ExpectEqsRitchieTypeOutcome {
                                                    ritchie_kind: Type(
                                                        Gn,
                                                    ),
                                                    parameter_contracted_tys: [
                                                        Variadic(
                                                            FluffyRitchieVariadicParameter {
                                                                contract: Pure,
                                                                ty: FluffyTerm {
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
                                                            },
                                                        ),
                                                        Keyed(
                                                            FluffyRitchieKeyedParameter {
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 295,
                                                                        },
                                                                    ),
                                                                ),
                                                                contract: Pure,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
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
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Hollow(
                                                            HollowTerm(
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 40,
                                    src: ExpectationSource {
                                        syn_expr_idx: 36,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                            EthTerm(`FermiMatchResult`),
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
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 41,
                                    src: ExpectationSource {
                                        syn_expr_idx: 37,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                            EthTerm(`f32`),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 42,
                                    src: ExpectationSource {
                                        syn_expr_idx: 38,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                            EthTerm(`FermiMatchResult`),
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
                                                EthTerm(`f32`),
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
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 44,
                                    src: ExpectationSource {
                                        syn_expr_idx: 40,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                            EthTerm(`FermiMatchResult`),
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
                                    expectee: FluffyTerm {
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
                                            EthTerm(`f32`),
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
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`i32`),
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
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`i32`),
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
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`OneVsAllResult MnistLabel Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 47,
                                    src: ExpectationSource {
                                        syn_expr_idx: 43,
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
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 48,
                                    src: ExpectationSource {
                                        syn_expr_idx: 43,
                                        kind: Expectation(
                                            47,
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
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 49,
                                    src: ExpectationSource {
                                        syn_expr_idx: 43,
                                        kind: Expectation(
                                            47,
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
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 50,
                                    src: ExpectationSource {
                                        syn_expr_idx: 44,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`unit`),
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
                                    idx: 51,
                                    src: ExpectationSource {
                                        syn_expr_idx: 45,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                            EthTerm(`FermiMatchResult`),
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
                                        syn_expr_idx: 46,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                            EthTerm(`f32`),
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
                                                            3,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 53,
                                    src: ExpectationSource {
                                        syn_expr_idx: 47,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 54,
                                    src: ExpectationSource {
                                        syn_expr_idx: 48,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                    idx: 55,
                                    src: ExpectationSource {
                                        syn_expr_idx: 49,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash ConnectedComponent`),
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
                                    idx: 56,
                                    src: ExpectationSource {
                                        syn_expr_idx: 50,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`EffHoles`),
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
                                    idx: 57,
                                    src: ExpectationSource {
                                        syn_expr_idx: 51,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec Option Leash RawContour`),
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
                                    idx: 58,
                                    src: ExpectationSource {
                                        syn_expr_idx: 52,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                9,
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
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 59,
                                    src: ExpectationSource {
                                        syn_expr_idx: 53,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                9,
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
                                    idx: 60,
                                    src: ExpectationSource {
                                        syn_expr_idx: 53,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 61,
                                    src: ExpectationSource {
                                        syn_expr_idx: 54,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                        syn_expr_idx: 55,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash ConnectedComponent`),
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
                                        syn_expr_idx: 56,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`EffHoles`),
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
                                        syn_expr_idx: 57,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec Option Leash RawContour`),
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
                                        syn_expr_idx: 58,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                10,
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
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 66,
                                    src: ExpectationSource {
                                        syn_expr_idx: 59,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                10,
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
                                    idx: 67,
                                    src: ExpectationSource {
                                        syn_expr_idx: 59,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 68,
                                    src: ExpectationSource {
                                        syn_expr_idx: 60,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                    idx: 69,
                                    src: ExpectationSource {
                                        syn_expr_idx: 61,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash ConnectedComponent`),
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
                                    idx: 70,
                                    src: ExpectationSource {
                                        syn_expr_idx: 62,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`EffHoles`),
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
                                    idx: 71,
                                    src: ExpectationSource {
                                        syn_expr_idx: 63,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Vec Option Leash RawContour`),
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
                                    idx: 72,
                                    src: ExpectationSource {
                                        syn_expr_idx: 64,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                11,
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
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 73,
                                    src: ExpectationSource {
                                        syn_expr_idx: 65,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                11,
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
                                    idx: 74,
                                    src: ExpectationSource {
                                        syn_expr_idx: 65,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 75,
                                    src: ExpectationSource {
                                        syn_expr_idx: 66,
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
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 76,
                                    src: ExpectationSource {
                                        syn_expr_idx: 67,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash RawContour`),
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
                                    idx: 77,
                                    src: ExpectationSource {
                                        syn_expr_idx: 68,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`BoundingBox`),
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
                                    idx: 78,
                                    src: ExpectationSource {
                                        syn_expr_idx: 72,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 79,
                                    src: ExpectationSource {
                                        syn_expr_idx: 69,
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
                                            EthTerm(`Option Leash RawContour`),
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
                                    idx: 80,
                                    src: ExpectationSource {
                                        syn_expr_idx: 70,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash RawContour`),
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
                                    idx: 81,
                                    src: ExpectationSource {
                                        syn_expr_idx: 71,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`BoundingBox`),
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
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 82,
                                    src: ExpectationSource {
                                        syn_expr_idx: 73,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 83,
                                    src: ExpectationSource {
                                        syn_expr_idx: 74,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 84,
                                    src: ExpectationSource {
                                        syn_expr_idx: 75,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash LineSegmentSketch`),
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
                                    idx: 85,
                                    src: ExpectationSource {
                                        syn_expr_idx: 76,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`BoundingBox`),
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
                                    idx: 86,
                                    src: ExpectationSource {
                                        syn_expr_idx: 79,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 87,
                                    src: ExpectationSource {
                                        syn_expr_idx: 77,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`Leash LineSegmentSketch`),
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
                                    idx: 88,
                                    src: ExpectationSource {
                                        syn_expr_idx: 78,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`BoundingBox`),
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
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 89,
                                    src: ExpectationSource {
                                        syn_expr_idx: 80,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 90,
                                    src: ExpectationSource {
                                        syn_expr_idx: 81,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 91,
                                    src: ExpectationSource {
                                        syn_expr_idx: 82,
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
                                            EthTerm(`f32`),
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
                                                            5,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 92,
                                    src: ExpectationSource {
                                        syn_expr_idx: 83,
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
                                            EthTerm(`f32`),
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
                                    idx: 93,
                                    src: ExpectationSource {
                                        syn_expr_idx: 84,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 94,
                                    src: ExpectationSource {
                                        syn_expr_idx: 85,
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
                                            EthTerm(`f32`),
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
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 95,
                                    src: ExpectationSource {
                                        syn_expr_idx: 86,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`f32`),
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
                                    idx: 96,
                                    src: ExpectationSource {
                                        syn_expr_idx: 87,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EthTerm(`bool`),
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
                                    idx: 97,
                                    src: ExpectationSource {
                                        syn_expr_idx: 88,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                            EthTerm(`FermiMatchResult`),
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
                                    idx: 98,
                                    src: ExpectationSource {
                                        syn_expr_idx: 89,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                            EthTerm(`f32`),
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
                                            base: FluffyTermBase::Ethereal(
                                                EthTerm(`OneVsAll MnistLabel Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 99,
                                    src: ExpectationSource {
                                        syn_expr_idx: 90,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                14,
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
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 100,
                                    src: ExpectationSource {
                                        syn_expr_idx: 90,
                                        kind: Expectation(
                                            99,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                12,
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
                                                EthTerm(`Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 101,
                                    src: ExpectationSource {
                                        syn_expr_idx: 90,
                                        kind: Expectation(
                                            99,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                13,
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
                                                EthTerm(`OneVsAll MnistLabel Zero`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 102,
                                    src: ExpectationSource {
                                        syn_expr_idx: 91,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                14,
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
                EthTerm(`OneVsAll MnistLabel Zero`),
            ),
            self_ty: None,
        },
    },
]