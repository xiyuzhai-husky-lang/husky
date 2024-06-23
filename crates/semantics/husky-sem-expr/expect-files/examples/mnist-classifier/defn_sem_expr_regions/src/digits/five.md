```rust
[
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath(`mnist_classifier::digits::five::is_five`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath(`mnist_classifier::digits::five::is_five`),
            ),
            place_registry: PlaceRegistry {
                infos: [],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 265,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath(`malamute::OneVsAll::Yes`),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
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
                                                                    0,
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
                                                                    1,
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
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
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
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
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
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
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
                                    sem_expr_idx: SemExprIdx(
                                        0,
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
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            2,
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
                    1,
                    (
                        SemExprIdx(
                            1,
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
                inherited_syn_symbol_map: [],
                current_variable_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
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
                                        0,
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
                                        0,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Five`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Five`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Five`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    refined_path: Right(
                                        OtherTypePath(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll MnistLabel Five`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 2,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAll MnistLabel Five`),
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
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
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
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expectation(
                                            0,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
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
                                                EthTerm(`Five`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expectation(
                                            0,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAll MnistLabel Five`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
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
                EthTerm(`OneVsAll MnistLabel Five`),
            ),
            self_ty: None,
        },
    },
]
```