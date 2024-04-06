```rust
[
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemPath::MajorItem(
                MajorItemPath::Trait(
                    TraitPath(`core::clone::Clone`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Decl(
                ItemPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::clone::Clone`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [],
                next: 0,
            },
            sem_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sem_stmt_arena: SemaStmtArena(
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
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EthTerm(`Self`),
            ),
        },
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemPath::ImplBlock(
                ImplBlockPath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockPath(`#derive _ as core::clone::Clone(0)`),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Decl(
                ItemPath::ImplBlock(
                    ImplBlockPath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath(`#derive _ as core::clone::Clone(0)`),
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
                                        MajorItemPath::Trait(
                                            TraitPath(`core::clone::Clone`),
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
            sem_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sem_expr_roots: [
                (
                    0,
                    (
                        SemaExprIdx(
                            0,
                        ),
                        SynExprRootKind::Trait,
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
                        0,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Clone`),
                            ),
                        },
                    ),
                ),
            ],
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
        },
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemPath::AssocItem(
                AssocItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        `<#derive _ as core::clone::Clone(0)>::clone`,
                        TraitItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: RegionPath::Decl(
                ItemPath::AssocItem(
                    AssocItemPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<#derive _ as core::clone::Clone(0)>::clone`,
                            TraitItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    SelfValue,
                ],
                next: 1,
            },
            sem_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::SelfType(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                ),
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
                                    0,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
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
                    data: [],
                },
            ),
            sem_expr_roots: [
                (
                    0,
                    (
                        SemaExprIdx(
                            0,
                        ),
                        SynExprRootKind::ReturnType,
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
                        0,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Self`),
                            ),
                        },
                    ),
                ),
            ],
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
                                expectation: Expectation::EqsSort(
                                    ExpectSort {
                                        smallest_universe: Universe(
                                            1,
                                        ),
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
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EthTerm(`Self`),
            ),
        },
    },
]
```