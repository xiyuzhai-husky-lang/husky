```rust
[
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath::AssocItem(
                AssocItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        `<malamute::OneVsAll as core::default::Default(0)>::default`,
                        TraitItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath::AssocItem(
                    AssocItemPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<malamute::OneVsAll as core::default::Default(0)>::default`,
                            TraitItemKind::AssocRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
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
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 6,
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
                                                            value: 6,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: None,
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
                inherited_syn_symbol_map: [
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
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Label`),
                            ),
                        },
                    ),
                ],
                current_variable_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`Label`),
                        ),
                    },
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`label`),
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
                                        0,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Label`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Label`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Label`),
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
                                                EthTerm(`label`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`label`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`label`),
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
                                    EthTerm(`OneVsAll Label label`),
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
                                                EthTerm(`OneVsAll Label label`),
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
                                                EthTerm(`Label`),
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
                                                EthTerm(`label`),
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
                                                EthTerm(`OneVsAll Label label`),
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
                EthTerm(`OneVsAll Label label`),
            ),
            self_ty: Some(
                EthTerm(`OneVsAll Label label`),
            ),
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath::AssocItem(
                AssocItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                        TraitItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath::AssocItem(
                    AssocItemPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                            TraitItemKind::AssocRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
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
                                SemExprData::InheritedSynSymbol {
                                    ident: `one_vs_all`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `one_vs_all`,
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
                                        EthTerm(`OneVsAll Label label`),
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
                                            EthTerm(`OneVsAll Label label`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 112,
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
                                                            value: 112,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: None,
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`B`),
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
                                                    EthSymbolicVariable(`C`),
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
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 7,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 2,
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
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: None,
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`Label`),
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
                                            6,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
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
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedSynSymbol {
                                    ident: `label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Template(
                                        InheritedTemplateVariable::Constant {
                                            ident: `label`,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Label`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Label`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        2,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            4,
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
                                                                expectee_quary: Compterm,
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
                                    4,
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
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        1,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
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
                                        23,
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
                                    5,
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
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 13,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 111,
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
                                                            value: 111,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: None,
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`B`),
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
                                                (
                                                    EthSymbolicVariable(`C`),
                                                    FlyTermSymbolResolution::Explicit(
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
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
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
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Unit {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
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
                                    9,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        6,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            8,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    7,
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
                                        39,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                9,
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
                                            2..3,
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
                                    13,
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
                                        5,
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
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    sem_expr_idx: SemExprIdx(
                                        8,
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
                                            9,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Match {
                                    match_token: MatchRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    match_opd: SemExprIdx(
                                        0,
                                    ),
                                    match_contract: Pure,
                                    eol_with_token: EolWithRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    case_branches: [
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            case_pattern_sem_obelisk: CaseVariableObelisk {
                                                syn_pattern_root: CaseSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..0,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                            ),
                                            stmts: SemStmtIdxRange(
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        },
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    24,
                                                ),
                                            ),
                                            case_pattern_sem_obelisk: CaseVariableObelisk {
                                                syn_pattern_root: CaseSynPatternRoot {
                                                    syn_pattern_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..0,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    28,
                                                ),
                                            ),
                                            stmts: SemStmtIdxRange(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                    ],
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
                    9,
                    (
                        SemExprIdx(
                            9,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
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
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Label`),
                            ),
                        },
                    ),
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
                                EthTerm(`OneVsAll Label label`),
                            ),
                        },
                    ),
                ],
                current_variable_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`Label`),
                        ),
                    },
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`label`),
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
                                        1,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Hol(
                                                    HolTerm(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Class Label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        1,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`unit`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        OtherTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
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
                                    EthTerm(`ControlFlow Class Label unit`),
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
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((move  Class Label) -> ControlFlow Class Label unit`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: Some(
                                                Compterm,
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Label`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                place: Some(
                                                    Compterm,
                                                ),
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Label`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`malamute::Class`, `Enum`),
                                    refined_path: Right(
                                        OtherTypePath(
                                            TypePath(`malamute::Class`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
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
                                    EthTerm(`Class Label`),
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
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((move  Label) -> Class Label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        6,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Class Label`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Class Label`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Class Label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        6,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`unit`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        OtherTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    8,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`ControlFlow Class Label unit`),
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
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            8,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((move  unit) -> ControlFlow Class Label unit`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 9,
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
                                            EthTerm(`OneVsAll Label label`),
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
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
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
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Hol(
                                                            HolTerm(
                                                                2,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
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
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
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
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Hol(
                                                            HolTerm(
                                                                5,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                4,
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
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    4,
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
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Label`),
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
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
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
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                5,
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`ControlFlow Class Label unit`),
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
                                                EthTerm(`Class Label`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expectation(
                                            5,
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
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expectation(
                                            5,
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
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
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
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Hol(
                                                            HolTerm(
                                                                9,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                8,
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
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    8,
                                                ),
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
                                        place: None,
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
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`ControlFlow Class Label unit`),
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
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                9,
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
                                                EthTerm(`Class Label`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expectation(
                                            10,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expectation(
                                            10,
                                        ),
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
                                                EthTerm(`ControlFlow Class Label unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
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
                EthTerm(`ControlFlow Class Label unit`),
            ),
            self_ty: Some(
                EthTerm(`Class Label`),
            ),
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath::AssocItem(
                AssocItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                        TraitItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath::AssocItem(
                    AssocItemPath::TraitForTypeItem(
                        TraitForTypeItemPath(
                            `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                            TraitItemKind::AssocRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
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
                                SemExprData::InheritedSynSymbol {
                                    ident: `one_vs_all_result`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `one_vs_all_result`,
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
                                        EthTerm(`OneVsAllResult Label label`),
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
                                            EthTerm(`OneVsAllResult Label label`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 112,
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
                                                            value: 112,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: None,
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`B`),
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
                                                    EthSymbolicVariable(`C`),
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
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 7,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 5,
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
                                                            value: 5,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: None,
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`Label`),
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
                                                (
                                                    EthSymbolicVariable(`label`),
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
                                            6,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
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
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        1,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    2,
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
                                        20,
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
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 13,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 112,
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
                                                            value: 112,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: None,
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`B`),
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
                                                (
                                                    EthSymbolicVariable(`C`),
                                                    FlyTermSymbolResolution::Explicit(
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
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
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
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 15,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 6,
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
                                                            value: 6,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: None,
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`Label`),
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
                                                (
                                                    EthSymbolicVariable(`label`),
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
                                            13,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
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
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        4,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            7,
                                                        ),
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
                                        37,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 21,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 111,
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
                                                            value: 111,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: None,
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`B`),
                                                    FlyTermSymbolResolution::Explicit(
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
                                                (
                                                    EthSymbolicVariable(`C`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    15,
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
                                            17,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                17,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Unit {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
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
                                    16,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        7,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            15,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    8,
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
                                        53,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            16,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                16,
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
                                            3..4,
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
                                    20,
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
                                        3,
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
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    sem_expr_idx: SemExprIdx(
                                        6,
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
                                            9,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    sem_expr_idx: SemExprIdx(
                                        9,
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
                                            16,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Match {
                                    match_token: MatchRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    match_opd: SemExprIdx(
                                        0,
                                    ),
                                    match_contract: Pure,
                                    eol_with_token: EolWithRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    case_branches: [
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            case_pattern_sem_obelisk: CaseVariableObelisk {
                                                syn_pattern_root: CaseSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..0,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                            ),
                                            stmts: SemStmtIdxRange(
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        },
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    21,
                                                ),
                                            ),
                                            case_pattern_sem_obelisk: CaseVariableObelisk {
                                                syn_pattern_root: CaseSynPatternRoot {
                                                    syn_pattern_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..0,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    25,
                                                ),
                                            ),
                                            stmts: SemStmtIdxRange(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    38,
                                                ),
                                            ),
                                            case_pattern_sem_obelisk: CaseVariableObelisk {
                                                syn_pattern_root: CaseSynPatternRoot {
                                                    syn_pattern_idx: 2,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..0,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    42,
                                                ),
                                            ),
                                            stmts: SemStmtIdxRange(
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        },
                                    ],
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
                    10,
                    (
                        SemExprIdx(
                            10,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 24,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 24,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 24,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
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
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                Compterm,
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Label`),
                            ),
                        },
                    ),
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
                                EthTerm(`OneVsAllResult Label label`),
                            ),
                        },
                    ),
                ],
                current_variable_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`Label`),
                        ),
                    },
                    FlyTerm {
                        place: None,
                        base: FlyTermBase::Eth(
                            EthTerm(`label`),
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
                                        1,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Hol(
                                                    HolTerm(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll Label label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        1,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`unit`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        OtherTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
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
                                    EthTerm(`ControlFlow OneVsAll Label label unit`),
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
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((move  OneVsAll Label label) -> ControlFlow OneVsAll Label label unit`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Label`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Label`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        2,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`label`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`label`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`label`),
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
                                                    4,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll Label label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        4,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    13,
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Hol(
                                                    HolTerm(
                                                        13,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll Label label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        4,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`unit`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        OtherTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    8,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`ControlFlow OneVsAll Label label unit`),
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
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            7,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((move  OneVsAll Label label) -> ControlFlow OneVsAll Label label unit`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        5,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Label`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Label`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        5,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`label`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`label`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`label`),
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
                                                    11,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    12,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll Label label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        7,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAll Label label`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`OneVsAll Label label`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll Label label`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        7,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`unit`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        OtherTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    14,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    15,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`ControlFlow OneVsAll Label label unit`),
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
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            15,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                16,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((move  unit) -> ControlFlow OneVsAll Label label unit`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 16,
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
                                            EthTerm(`OneVsAllResult Label label`),
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
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
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
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Hol(
                                                            HolTerm(
                                                                2,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
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
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
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
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                6,
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
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`ControlFlow OneVsAll Label label unit`),
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
                                                EthTerm(`OneVsAll Label label`),
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
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expectation(
                                            3,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Label`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expectation(
                                            4,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`label`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expectation(
                                            4,
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
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
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
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Hol(
                                                            HolTerm(
                                                                9,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                7,
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
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
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
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`ControlFlow OneVsAll Label label unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                9,
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
                                                EthTerm(`OneVsAll Label label`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expectation(
                                            10,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expectation(
                                            10,
                                        ),
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
                                                EthTerm(`Label`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expectation(
                                            11,
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`label`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expectation(
                                            11,
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
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                17,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Hol(
                                                            HolTerm(
                                                                16,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                15,
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
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    15,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
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
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`ControlFlow OneVsAll Label label unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                16,
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
                                                EthTerm(`OneVsAll Label label`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expectation(
                                            17,
                                        ),
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
                                                EthTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expectation(
                                            17,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                15,
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
                                                EthTerm(`ControlFlow OneVsAll Label label unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
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
                EthTerm(`ControlFlow OneVsAll Label label unit`),
            ),
            self_ty: Some(
                EthTerm(`OneVsAll Label label`),
            ),
        },
    },
]
```