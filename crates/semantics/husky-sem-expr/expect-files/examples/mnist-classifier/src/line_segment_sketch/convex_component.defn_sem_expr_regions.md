```rust
[
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath(`<mnist_classifier::line_segment_sketch::convex_component::ConvexComponent as core::visual::Visualize(0)>::visualize`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath(`<mnist_classifier::line_segment_sketch::convex_component::ConvexComponent as core::visual::Visualize(0)>::visualize`),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::SelfValue,
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::SelfValue(
                                    RegionalTokenIdx(
                                        1,
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`ConvexComponent`),
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
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`ConvexComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        0,
                                    ),
                                    self_argument_ty: FlyTerm {
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`ConvexComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `line_segments`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    dispatch: FlyInstanceDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        },
                                        signature: FieldFlySignature::PropsStruct {
                                            self_ty: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`ConvexComponent`),
                                                ),
                                            },
                                            ty: FlyTerm {
                                                quary: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Leash CyclicSlice LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash CyclicSlice LineSegmentStroke`),
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
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash CyclicSlice LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodRitchieCall {
                                    self_argument: SemExprIdx(
                                        1,
                                    ),
                                    self_ty: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`CyclicSlice LineSegmentStroke`),
                                        ),
                                    },
                                    self_contract: Contract::Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `visualize`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                    dispatch: FlyInstanceDispatch {
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
                                                place: None,
                                            },
                                        },
                                        signature: MethodFlySignature::TraitForTypeMethodRitchie(
                                            TraitForTypeMethodRitchieFlySignature {
                                                path: TraitForTypeItemPath(
                                                    `<#derive _ as core::visual::Visualize(0)>::visualize`,
                                                    TraitItemKind::MethodRitchie(
                                                        RitchieItemKind::Fn,
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Contract::Pure,
                                                    ty: FlyTerm {
                                                        quary: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`CyclicSlice LineSegmentStroke`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                self_ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`CyclicSlice LineSegmentStroke`),
                                                    ),
                                                },
                                                return_ty: FlyTerm {
                                                    quary: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Visual`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath(`<#derive _ as core::visual::Visualize(0)>::visualize`),
                                                    context_itd: EthTermContextItd {
                                                        task_ty: Some(
                                                            EthTerm(`MnistTask`),
                                                        ),
                                                    },
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place: None,
                                                        },
                                                    },
                                                    variable_map: [
                                                        (
                                                            EthSymbolicVariable(`Self`, `nil`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    quary: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`CyclicSlice LineSegmentStroke`),
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
                                        6,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Visual`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Visual`),
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
                                        EthTerm(`Visual`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Visual`),
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
                                        2,
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
                                        EthTerm(`Visual`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    3,
                    (
                        SemExprIdx(
                            3,
                        ),
                        SynExprRootKind::RootBody,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_variable_map: [],
                current_variable_map: [],
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
                                        quary: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`ConvexComponent`),
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
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash CyclicSlice LineSegmentStroke`),
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
                                                EthTerm(`Visual`),
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
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Visual`),
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
                                                EthTerm(`Visual`),
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
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Visual`),
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
                EthTerm(`Visual`),
            ),
            self_ty: Some(
                EthTerm(`ConvexComponent`),
            ),
            self_value_ty: Some(
                FlyTerm {
                    quary: Some(
                        StackPure {
                            place: Idx(
                                PlaceIdx(0),
                            ),
                        },
                    ),
                    base: FlyTermBase::Eth(
                        EthTerm(`ConvexComponent`),
                    ),
                },
            ),
            context_itd: EthTermContextItd {
                task_ty: Some(
                    EthTerm(`MnistTask`),
                ),
            },
        },
    },
]
```