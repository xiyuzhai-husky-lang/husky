```rust
[
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`core::mem::Ref`, `Extern`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::mem::Ref`, `Extern`),
                    ),
                ),
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
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Lifetime`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Type`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`'a`),
                        ),
                    },
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`E`),
                        ),
                    },
                ],
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
                EthTerm(`Ref 'a E`),
            ),
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`core::mem::RefMut`, `Extern`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::mem::RefMut`, `Extern`),
                    ),
                ),
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
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Lifetime`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Type`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`'a`),
                        ),
                    },
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`E`),
                        ),
                    },
                ],
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
                EthTerm(`RefMut 'a E`),
            ),
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`core::mem::Leash`, `Extern`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::mem::Leash`, `Extern`),
                    ),
                ),
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
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Type`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`E`),
                        ),
                    },
                ],
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
                EthTerm(`Leash E`),
            ),
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`core::mem::At`, `Extern`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::mem::At`, `Extern`),
                    ),
                ),
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
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Place`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Type`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`'α`),
                        ),
                    },
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`E`),
                        ),
                    },
                ],
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
                EthTerm(`At 'α E`),
            ),
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath::ImplBlock(
                ImplBlockPath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockPath(`core::mem::Leash as core::marker::Copy(0)`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath::ImplBlock(
                    ImplBlockPath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath(`core::mem::Leash as core::marker::Copy(0)`),
                    ),
                ),
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
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::mem::Leash`, `Extern`),
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
                                        EthTerm(`covariant Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`covariant Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `E`,
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::TemplateParameter {
                                        template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                            ident_token: IdentRegionalToken {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    3,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
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
                                    function_sem_expr_idx: SemExprIdx(
                                        1,
                                    ),
                                    argument_sem_expr_idx: SemExprIdx(
                                        2,
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
                                    3,
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
                (
                    3,
                    (
                        SemExprIdx(
                            3,
                        ),
                        SynExprRootKind::SelfType,
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
                (
                    SemExprIdx(
                        1,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Leash`),
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
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`E`),
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
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`Leash E`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Type`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`E`),
                        ),
                    },
                ],
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
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::Sort,
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
                                            EthTerm(`covariant Type -> Type`),
                                        ),
                                    },
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
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
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
                                    idx: 3,
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
                EthTerm(`Leash E`),
            ),
        },
    },
]
```