```rust
[
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath(`core::marker::Copy`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath(`core::marker::Copy`),
            ),
            place_registry: PlaceRegistry {
                infos: [],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [],
                },
            ),
            sem_expr_roots: [],
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
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EthTerm(`Self`),
            ),
            context_itd: EthSignatureBuilderContextItd {
                context: EthSignatureBuilderContext {
                    task_ty: None,
                },
            },
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath(`core::marker::Sized`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath(`core::marker::Sized`),
            ),
            place_registry: PlaceRegistry {
                infos: [],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [],
                },
            ),
            sem_expr_roots: [],
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
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EthTerm(`Self`),
            ),
            context_itd: EthSignatureBuilderContextItd {
                context: EthSignatureBuilderContext {
                    task_ty: None,
                },
            },
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath(`#derive _ as core::marker::Copy(0)`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath(`#derive _ as core::marker::Copy(0)`),
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
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::marker::Copy`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Trait`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Trait`),
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
                    data: [],
                },
            ),
            sem_expr_roots: [
                (
                    0,
                    (
                        SemExprIdx(
                            0,
                        ),
                        SynExprRootKind::PrimalTrait,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [
                (
                    SemExprIdx(
                        0,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Copy`),
                            ),
                        },
                    ),
                ),
            ],
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
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Trait`),
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
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EthTerm(`Self`),
            ),
            context_itd: EthSignatureBuilderContextItd {
                context: EthSignatureBuilderContext {
                    task_ty: None,
                },
            },
        },
    },
]
```