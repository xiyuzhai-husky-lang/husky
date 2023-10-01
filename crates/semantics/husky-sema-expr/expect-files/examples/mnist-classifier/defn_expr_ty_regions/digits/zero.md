[
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 25,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 16,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 2,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 77,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 70,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 3,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            NewList {
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
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            FnCall {
                                function_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    2,
                                ),
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 70,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                2,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 72,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                4,
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
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 62,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        1..2,
                                    ),
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 62,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
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
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    5,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 62,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                6,
                SemaExprIdx(
                    6,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
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
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn( Leash Vec ConcaveComponent,  Vec fn( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`FermiMatchResult`),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        Application(
                                                                            EtherealTermApplication(
                                                                                Id {
                                                                                    value: 70,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        Application(
                                                                            EtherealTermApplication(
                                                                                Id {
                                                                                    value: 72,
                                                                                },
                                                                            ),
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                            EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec fn( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec fn( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
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
        return_ty: Some(
            EtherealTerm(`FermiMatchResult`),
        ),
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 287,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    2,
                                ),
                                inherited_symbol_idx: 1,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 287,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 71,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 350,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 59,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    6,
                                ),
                                Float(
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
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    2,
                                ),
                                opr: Closed(
                                    Add,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    5,
                                ),
                                ropd: SemaExprIdx(
                                    3,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    9,
                                ),
                                Float(
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
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Prefix {
                                opr: Minus,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    8,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    5,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    4,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                                ropd: SemaExprIdx(
                                    6,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 287,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                inherited_symbol_idx: 1,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 287,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 71,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    8,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 350,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 59,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Prefix {
                                opr: Minus,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    10,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    9,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    15,
                                ),
                                Float(
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
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    10,
                                ),
                                opr: Closed(
                                    Add,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    14,
                                ),
                                ropd: SemaExprIdx(
                                    11,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        1..3,
                                    ),
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
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
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    7,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    12,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                13,
                SemaExprIdx(
                    13,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
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
                                    NotNan(
                                        0.0,
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
                                    NotNan(
                                        140.0,
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
                                    NotNan(
                                        0.0,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash ConcaveComponent`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    4,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`f32`),
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
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            WrapInSome,
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
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            WrapInSome,
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
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 30,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Be {
                                src: SemaExprIdx(
                                    1,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 2,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    3,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 257,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 63,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    4,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    9,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 142,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 18,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    14,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    5,
                                ),
                                opr: Comparison(
                                    Eq,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    13,
                                ),
                                ropd: SemaExprIdx(
                                    6,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 3,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 26,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 84,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    8,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    20,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 353,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 447,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    23,
                                ),
                                current_symbol_idx: 2,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 2,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    25,
                                ),
                                Float(
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
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    10,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    24,
                                ),
                                ropd: SemaExprIdx(
                                    11,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 4,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 26,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 84,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    13,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    28,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                    signature: PropsStruct {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 86,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    31,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        0,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    14,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    30,
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
                                    32,
                                ),
                                index_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 85,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 85,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Be {
                                src: SemaExprIdx(
                                    16,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    33,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        3,
                                    ),
                                    variables: ArenaIdxRange(
                                        3..4,
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 5,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 71,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 104,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    18,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    37,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 142,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 18,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    39,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    40,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    42,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    19,
                                ),
                                opr: Comparison(
                                    Eq,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    41,
                                ),
                                ropd: SemaExprIdx(
                                    20,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 6,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 26,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 84,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    22,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    47,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                    signature: PropsStruct {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 86,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    50,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        0,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    23,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    49,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            24,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    51,
                                ),
                                index_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 85,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 85,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Suffix {
                                opd_sema_expr_idx: SemaExprIdx(
                                    25,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    52,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 71,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    26,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    53,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 303,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 53,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    55,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    56,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    27,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    57,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 353,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        58,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    59,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    60,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 450,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    62,
                                ),
                                current_symbol_idx: 4,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 4,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    64,
                                ),
                                Float(
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
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    29,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    63,
                                ),
                                ropd: SemaExprIdx(
                                    30,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 8,
                                path: TypeVariant(
                                    TypeVariantPath(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 30,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 9,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 25,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 16,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 10,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 77,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 70,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            NewList {
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    76,
                                ),
                                items: [],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    77,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            FnCall {
                                function_sema_expr_idx: SemaExprIdx(
                                    33,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    73,
                                ),
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 70,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                34,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    75,
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 72,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                35,
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
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 62,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 11,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 78,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 19,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 452,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    81,
                                ),
                                current_symbol_idx: 5,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 5,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 62,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    38,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    82,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 353,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        83,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 452,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    85,
                                ),
                                current_symbol_idx: 5,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 5,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 62,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    40,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    86,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 412,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        87,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 452,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    89,
                                ),
                                current_symbol_idx: 5,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 5,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 62,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    42,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    90,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 436,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    95,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        5,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            FnCall {
                                function_sema_expr_idx: SemaExprIdx(
                                    37,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    80,
                                ),
                                ritchie_parameter_argument_matches: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        [
                                            SemaVariadicCallListItem {
                                                argument_sema_expr_idx: SemaExprIdx(
                                                    39,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        84,
                                                    ),
                                                ),
                                            },
                                            SemaVariadicCallListItem {
                                                argument_sema_expr_idx: SemaExprIdx(
                                                    41,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        88,
                                                    ),
                                                ),
                                            },
                                            SemaVariadicCallListItem {
                                                argument_sema_expr_idx: SemaExprIdx(
                                                    43,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        92,
                                                    ),
                                                ),
                                            },
                                        ],
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 454,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 18,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            default: Some(
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
                                        },
                                        SemaKeyedCallListItem {
                                            key_regional_token_idx: RegionalTokenIdx(
                                                93,
                                            ),
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 454,
                                                    },
                                                ),
                                            ),
                                            argument_sema_expr_idx: SemaExprIdx(
                                                44,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    96,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        7,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Suffix {
                                opd_sema_expr_idx: SemaExprIdx(
                                    45,
                                ),
                                opr: Unveil,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    97,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 452,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    99,
                                ),
                                current_symbol_idx: 5,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 5,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 62,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    47,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    100,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 353,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    103,
                                ),
                                Float(
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
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    48,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    102,
                                ),
                                ropd: SemaExprIdx(
                                    49,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 12,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    51,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    106,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 259,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        107,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 46,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    52,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    108,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 46,
                                        },
                                    ),
                                    signature: PropsStruct {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 66,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 66,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    111,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        9,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    53,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    110,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            54,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    112,
                                ),
                                index_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 65,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Be {
                                src: SemaExprIdx(
                                    55,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    113,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        6,
                                    ),
                                    variables: ArenaIdxRange(
                                        6..7,
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 13,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    57,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    117,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 259,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        118,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 46,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    58,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    119,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        120,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 46,
                                        },
                                    ),
                                    signature: PropsStruct {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 66,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 66,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    122,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        0,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        10,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    59,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    121,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            60,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    123,
                                ),
                                index_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 65,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Be {
                                src: SemaExprIdx(
                                    61,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    124,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        7,
                                    ),
                                    variables: ArenaIdxRange(
                                        7..8,
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 14,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    63,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    130,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 259,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        131,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 46,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    64,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    132,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        133,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 46,
                                        },
                                    ),
                                    signature: PropsStruct {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 66,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 66,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    135,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        0,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        11,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    65,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    134,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            66,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    136,
                                ),
                                index_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 65,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 455,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    140,
                                ),
                                current_symbol_idx: 8,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 8,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 65,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Suffix {
                                opd_sema_expr_idx: SemaExprIdx(
                                    68,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    141,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 46,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    69,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    142,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        143,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 48,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 55,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 55,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    70,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    144,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 298,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        145,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    146,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    147,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 455,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    149,
                                ),
                                current_symbol_idx: 8,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 8,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 65,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Suffix {
                                opd_sema_expr_idx: SemaExprIdx(
                                    72,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    150,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 46,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    73,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    151,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        152,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 48,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 55,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 55,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    74,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    153,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 297,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        154,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    155,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    156,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    71,
                                ),
                                opr: Closed(
                                    Sub,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    148,
                                ),
                                ropd: SemaExprIdx(
                                    75,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 15,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 76,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 73,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    77,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    161,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        162,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 58,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 55,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 55,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    78,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    163,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 298,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        164,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    165,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    166,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 16,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 76,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 73,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    80,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    169,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        170,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [
                                            Leash,
                                        ],
                                        final_place: Leashed,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 58,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 55,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 55,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    81,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    171,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 297,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        172,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    173,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    174,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    79,
                                ),
                                opr: Closed(
                                    Sub,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    167,
                                ),
                                ropd: SemaExprIdx(
                                    82,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 53,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    178,
                                ),
                                current_symbol_idx: 9,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 9,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 158,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    180,
                                ),
                                current_symbol_idx: 10,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 10,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    84,
                                ),
                                opr: Closed(
                                    Div,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    179,
                                ),
                                ropd: SemaExprIdx(
                                    85,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 457,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    182,
                                ),
                                current_symbol_idx: 11,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 11,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    184,
                                ),
                                Float(
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
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    87,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    183,
                                ),
                                ropd: SemaExprIdx(
                                    88,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 452,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    188,
                                ),
                                current_symbol_idx: 5,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 5,
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 62,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    90,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    189,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 353,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        190,
                                    ),
                                },
                                field_dispatch: FluffyFieldDyanmicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    ty_path: TypePath(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                    signature: Memoized {
                                        ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 18,
                                path: TypeVariant(
                                    TypeVariantPath(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 30,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        8..22,
                                    ),
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 30,
                                            },
                                        ),
                                    ),
                                ),
                            },
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        2,
                                    ),
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        18,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    9,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    12,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    17,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    21,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        4,
                                    ),
                                    variables: ArenaIdxRange(
                                        4..5,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        45,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    28,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    31,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Return {
                                return_token: ReturnRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        65,
                                    ),
                                },
                                result: SemaExprIdx(
                                    32,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 3,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    2,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            IfElse {
                                sema_if_branch: SemaIfBranch {
                                    if_token: IfRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        7,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                15,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..8,
                                        ),
                                    ),
                                },
                                sema_elif_branches: [],
                                sema_else_branch: None,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 3,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        69,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        5,
                                    ),
                                    variables: ArenaIdxRange(
                                        5..6,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        71,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    36,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    46,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        98,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    50,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        104,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    56,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        115,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    62,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        126,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        8,
                                    ),
                                    variables: ArenaIdxRange(
                                        8..9,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        128,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    67,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        137,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        9,
                                    ),
                                    variables: ArenaIdxRange(
                                        9..10,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        139,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    76,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        157,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        10,
                                    ),
                                    variables: ArenaIdxRange(
                                        10..11,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        159,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    83,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        175,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        11,
                                    ),
                                    variables: ArenaIdxRange(
                                        11..12,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        177,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    86,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        181,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    89,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        185,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        12,
                                    ),
                                    variables: ArenaIdxRange(
                                        12..13,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        187,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    91,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    92,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 30,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                93,
                SemaExprIdx(
                    93,
                ),
            ),
        ],
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 27,
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
                                        Id {
                                            value: 28,
                                        },
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 85,
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
                                        Id {
                                            value: 28,
                                        },
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
                                        Id {
                                            value: 62,
                                        },
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 65,
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 65,
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 65,
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
                                        Id {
                                            value: 28,
                                        },
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
                                        Id {
                                            value: 28,
                                        },
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
                                        Id {
                                            value: 28,
                                        },
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
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
        ],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 27,
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
                                                Id {
                                                    value: 28,
                                                },
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
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 85,
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
                                                Id {
                                                    value: 28,
                                                },
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
                                                Id {
                                                    value: 62,
                                                },
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
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 65,
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
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 65,
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
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 65,
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
                                                Id {
                                                    value: 28,
                                                },
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
                                                Id {
                                                    value: 28,
                                                },
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
                                                Id {
                                                    value: 28,
                                                },
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
                                                Id {
                                                    value: 28,
                                                },
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
                    6,
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
                    11,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        1.5,
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
                    20,
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
                    24,
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
                    30,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        5.5,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    44,
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
                    49,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        3.0,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    54,
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
                    60,
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
                    66,
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
                    88,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        0.4,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash OneVsAll MnistLabel One`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`f32`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Option Leash ConcaveComponent`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`f32`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`FermiMatchResult`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Option Leash RawContour`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Option Leash RawContour`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Option Leash RawContour`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`f32`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`f32`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`f32`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`f32`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    15,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
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
                            data: Hole {
                                hole_source: Expr(
                                    24,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
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
                            data: Hole {
                                hole_source: Expectation(
                                    39,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 65,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 65,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Variable(
                                                EtherealTermVariable(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 454,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 18,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            default: Some(
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
                                        },
                                    ),
                                ],
                                return_ty: FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Variable(
                                                EtherealTermVariable(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                parameter_ty: FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                                return_ty: FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            4,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    39,
                                ),
                                hole_kind: Any,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeVariant(
                                                    TypeVariantPath(
                                                        Id {
                                                            value: 20,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`Zero`),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 65,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 65,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 454,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 18,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            default: Some(
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
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    54,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
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
                            data: Hole {
                                hole_source: Expr(
                                    60,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
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
                            data: Hole {
                                hole_source: Expr(
                                    66,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`usize`),
                            ),
                        },
                    ],
                    first_unresolved_term_idx: 2,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash OneVsAll MnistLabel One`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Vec ConnectedComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 20,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 21,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 22,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 23,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 24,
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
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 25,
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
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 25,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 26,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 28,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 29,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 30,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 31,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
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
                                            EtherealTerm(`OneVsAll MnistLabel Zero`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 34,
                                src: ExpectationSource {
                                    expr_idx: 32,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Zero`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                            meta: ExpectationState {
                                idx: 35,
                                src: ExpectationSource {
                                    expr_idx: 33,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn( Leash Vec ConcaveComponent,  Vec fn( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`FermiMatchResult`),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        Application(
                                                                            EtherealTermApplication(
                                                                                Id {
                                                                                    value: 70,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        Application(
                                                                            EtherealTermApplication(
                                                                                Id {
                                                                                    value: 72,
                                                                                },
                                                                            ),
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 34,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec fn( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 37,
                                src: ExpectationSource {
                                    expr_idx: 35,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec fn( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 38,
                                src: ExpectationSource {
                                    expr_idx: 36,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 39,
                                src: ExpectationSource {
                                    expr_idx: 37,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> (independent variable_ad_hoc_fmt: variable_ad_hoc_fmt) -> fn(f32, i32) -> OneVsAllResult variable_ad_hoc_fmt variable_ad_hoc_fmt`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm {
                                                            place: None,
                                                            base: Ethereal(
                                                                Variable(
                                                                    EtherealTermVariable(
                                                                        Id {
                                                                            value: 1,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        substitute: FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm {
                                                            place: None,
                                                            base: Ethereal(
                                                                Variable(
                                                                    EtherealTermVariable(
                                                                        Id {
                                                                            value: 2,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        substitute: FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    6,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: Ethereal(
                                                                    EntityPath(
                                                                        TypeOntology(
                                                                            TypePath(
                                                                                Id {
                                                                                    value: 28,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 454,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: Ethereal(
                                                                    EntityPath(
                                                                        TypeOntology(
                                                                            TypePath(
                                                                                Id {
                                                                                    value: 18,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            default: Some(
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
                            meta: ExpectationState {
                                idx: 40,
                                src: ExpectationSource {
                                    expr_idx: 38,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 41,
                                src: ExpectationSource {
                                    expr_idx: 39,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 42,
                                src: ExpectationSource {
                                    expr_idx: 40,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 43,
                                src: ExpectationSource {
                                    expr_idx: 41,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 44,
                                src: ExpectationSource {
                                    expr_idx: 42,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 45,
                                src: ExpectationSource {
                                    expr_idx: 43,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 46,
                                src: ExpectationSource {
                                    expr_idx: 44,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                            EtherealTerm(`OneVsAllResult MnistLabel Zero`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 47,
                                src: ExpectationSource {
                                    expr_idx: 45,
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
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 66,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 48,
                                src: ExpectationSource {
                                    expr_idx: 45,
                                    kind: Expectation(
                                        47,
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
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::Subtype(
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
                                        base: Ethereal(
                                            EntityPath(
                                                TypeVariant(
                                                    TypeVariantPath(
                                                        Id {
                                                            value: 20,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 49,
                                src: ExpectationSource {
                                    expr_idx: 45,
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
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 50,
                                src: ExpectationSource {
                                    expr_idx: 46,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 51,
                                src: ExpectationSource {
                                    expr_idx: 47,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 52,
                                src: ExpectationSource {
                                    expr_idx: 48,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 53,
                                src: ExpectationSource {
                                    expr_idx: 49,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 54,
                                src: ExpectationSource {
                                    expr_idx: 50,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 55,
                                src: ExpectationSource {
                                    expr_idx: 51,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 56,
                                src: ExpectationSource {
                                    expr_idx: 52,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`EffHoles`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 57,
                                src: ExpectationSource {
                                    expr_idx: 53,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 58,
                                src: ExpectationSource {
                                    expr_idx: 54,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 59,
                                src: ExpectationSource {
                                    expr_idx: 55,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 60,
                                src: ExpectationSource {
                                    expr_idx: 55,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 61,
                                src: ExpectationSource {
                                    expr_idx: 56,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 62,
                                src: ExpectationSource {
                                    expr_idx: 57,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 63,
                                src: ExpectationSource {
                                    expr_idx: 58,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`EffHoles`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 64,
                                src: ExpectationSource {
                                    expr_idx: 59,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 65,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            10,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 66,
                                src: ExpectationSource {
                                    expr_idx: 61,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            10,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 67,
                                src: ExpectationSource {
                                    expr_idx: 61,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 68,
                                src: ExpectationSource {
                                    expr_idx: 62,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 69,
                                src: ExpectationSource {
                                    expr_idx: 63,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConnectedComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 70,
                                src: ExpectationSource {
                                    expr_idx: 64,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`EffHoles`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 71,
                                src: ExpectationSource {
                                    expr_idx: 65,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 72,
                                src: ExpectationSource {
                                    expr_idx: 66,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            11,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 73,
                                src: ExpectationSource {
                                    expr_idx: 67,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            11,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 74,
                                src: ExpectationSource {
                                    expr_idx: 67,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 75,
                                src: ExpectationSource {
                                    expr_idx: 68,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 76,
                                src: ExpectationSource {
                                    expr_idx: 69,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 77,
                                src: ExpectationSource {
                                    expr_idx: 70,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 78,
                                src: ExpectationSource {
                                    expr_idx: 74,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 79,
                                src: ExpectationSource {
                                    expr_idx: 71,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 80,
                                src: ExpectationSource {
                                    expr_idx: 72,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash RawContour`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 81,
                                src: ExpectationSource {
                                    expr_idx: 73,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 82,
                                src: ExpectationSource {
                                    expr_idx: 75,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 83,
                                src: ExpectationSource {
                                    expr_idx: 76,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 84,
                                src: ExpectationSource {
                                    expr_idx: 77,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash LineSegmentSketch`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 85,
                                src: ExpectationSource {
                                    expr_idx: 78,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 86,
                                src: ExpectationSource {
                                    expr_idx: 81,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 87,
                                src: ExpectationSource {
                                    expr_idx: 79,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash LineSegmentSketch`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 88,
                                src: ExpectationSource {
                                    expr_idx: 80,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 89,
                                src: ExpectationSource {
                                    expr_idx: 82,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 90,
                                src: ExpectationSource {
                                    expr_idx: 83,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 91,
                                src: ExpectationSource {
                                    expr_idx: 84,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 92,
                                src: ExpectationSource {
                                    expr_idx: 85,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 93,
                                src: ExpectationSource {
                                    expr_idx: 86,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 94,
                                src: ExpectationSource {
                                    expr_idx: 87,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 95,
                                src: ExpectationSource {
                                    expr_idx: 88,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 96,
                                src: ExpectationSource {
                                    expr_idx: 89,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 97,
                                src: ExpectationSource {
                                    expr_idx: 90,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 98,
                                src: ExpectationSource {
                                    expr_idx: 91,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
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
                                            EtherealTerm(`OneVsAll MnistLabel Zero`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 99,
                                src: ExpectationSource {
                                    expr_idx: 92,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Zero`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                            EtherealTerm(`OneVsAll MnistLabel Zero`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 100,
                                src: ExpectationSource {
                                    expr_idx: 93,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Zero`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
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
        return_ty: Some(
            EtherealTerm(`OneVsAll MnistLabel Zero`),
        ),
        self_ty: None,
    },
]