[
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                                value: 67,
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
                            PrincipalEntityPath {
                                path_expr_idx: 4,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 68,
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
                            PrincipalEntityPath {
                                path_expr_idx: 5,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 69,
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
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    11,
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
                                                6,
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
                                    7,
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
                8,
                SemaExprIdx(
                    8,
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
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
                            path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
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
                                    4,
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
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    5,
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
                                        6,
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
                                    7,
                                ),
                                ritchie_parameter_argument_matches: [],
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 388,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    10,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    3,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 277,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
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
                                            value: 53,
                                        },
                                    ),
                                    signature: PropsStruct {
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
                                    14,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 119,
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
                                    4,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    13,
                                ),
                                ropd: SemaExprIdx(
                                    5,
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
                                            value: 388,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    15,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    7,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    16,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 277,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
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
                                            value: 53,
                                        },
                                    ),
                                    signature: PropsStruct {
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
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        1..4,
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                                condition: SemaExprIdx(
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
                                    8,
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
                9,
                SemaExprIdx(
                    9,
                ),
            ),
        ],
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
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
        ],
        pattern_symbol_ty_infos: ArenaMap {
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
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vector2d`),
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
                    entries: [],
                    first_unresolved_term_idx: 0,
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
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
                                idx: 5,
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
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
                            path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
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
                                    4,
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
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    5,
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
                                        6,
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
                                    7,
                                ),
                                ritchie_parameter_argument_matches: [],
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 388,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    10,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    3,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 277,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
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
                                            value: 53,
                                        },
                                    ),
                                    signature: PropsStruct {
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
                                    14,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 120,
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
                                    4,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    13,
                                ),
                                ropd: SemaExprIdx(
                                    5,
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
                                            value: 388,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    15,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    7,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    16,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 277,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
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
                                            value: 53,
                                        },
                                    ),
                                    signature: PropsStruct {
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
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        1..4,
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                                condition: SemaExprIdx(
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
                                    8,
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
                9,
                SemaExprIdx(
                    9,
                ),
            ),
        ],
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
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
        ],
        pattern_symbol_ty_infos: ArenaMap {
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
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vector2d`),
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
                    entries: [],
                    first_unresolved_term_idx: 0,
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
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
                                idx: 5,
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
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
                            path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                                    4,
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
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    5,
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
                                        6,
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
                                    7,
                                ),
                                ritchie_parameter_argument_matches: [],
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 388,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    10,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    3,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 276,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
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
                                            value: 53,
                                        },
                                    ),
                                    signature: PropsStruct {
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
                                    14,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 121,
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
                                    4,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    13,
                                ),
                                ropd: SemaExprIdx(
                                    5,
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
                                            value: 388,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    15,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    7,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    16,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 276,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
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
                                            value: 53,
                                        },
                                    ),
                                    signature: PropsStruct {
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
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        1..4,
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                                condition: SemaExprIdx(
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
                                    8,
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
                9,
                SemaExprIdx(
                    9,
                ),
            ),
        ],
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
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
        ],
        pattern_symbol_ty_infos: ArenaMap {
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
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vector2d`),
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
                    entries: [],
                    first_unresolved_term_idx: 0,
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
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
                                idx: 5,
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
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
                            path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                value: 28,
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
                                                value: 31,
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
                                                value: 40,
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
                                                value: 37,
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
                                    3,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        2,
                                    ),
                                    variables: ArenaIdxRange(
                                        2..3,
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
                                path_expr_idx: 3,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 57,
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
                                                value: 33,
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
                                    5,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    11,
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
                                path_expr_idx: 4,
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
                                    7,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    15,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        4,
                                    ),
                                    variables: ArenaIdxRange(
                                        4..5,
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
                                                value: 63,
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
                                                value: 39,
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
                                    9,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    19,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        5,
                                    ),
                                    variables: ArenaIdxRange(
                                        5..6,
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
                                path_expr_idx: 6,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 36,
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
                                                value: 29,
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
                                    11,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    23,
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
                                path_expr_idx: 7,
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
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    13,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    29,
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
                                        30,
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
                                    31,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
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
                            PrincipalEntityPath {
                                path_expr_idx: 8,
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
                                    15,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    37,
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
                                        38,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 259,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    40,
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
                                    17,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    41,
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
                                        42,
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
                                    44,
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
                                    18,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    43,
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
                                    45,
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
                                    20,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    46,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        9,
                                    ),
                                    variables: ArenaIdxRange(
                                        9..10,
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
                                path_expr_idx: 9,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 66,
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
                                    52,
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
                                        53,
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
                                    55,
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
                                    54,
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
                                    56,
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
                            PrincipalEntityPath {
                                path_expr_idx: 10,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 66,
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
                                    26,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    61,
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
                                        62,
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
                                    64,
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
                                        2,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    27,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    63,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            28,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    65,
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
                            PrincipalEntityPath {
                                path_expr_idx: 11,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 66,
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
                                    30,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    70,
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
                                        71,
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
                                    73,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        2,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        3,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    31,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    72,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            32,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    74,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 529,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    76,
                                ),
                                current_symbol_idx: 7,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 7,
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
                                    78,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        3,
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
                                    34,
                                ),
                                opr: Comparison(
                                    Leq,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    77,
                                ),
                                ropd: SemaExprIdx(
                                    35,
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
                                    37,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    83,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 247,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        84,
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
                                    39,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    87,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 246,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        88,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    38,
                                ),
                                opr: Closed(
                                    Sub,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    85,
                                ),
                                ropd: SemaExprIdx(
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
                                            value: 478,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    90,
                                ),
                                current_symbol_idx: 13,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 13,
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
                                    92,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 122,
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
                                    42,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    91,
                                ),
                                ropd: SemaExprIdx(
                                    43,
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
                                            value: 529,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    94,
                                ),
                                current_symbol_idx: 7,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 7,
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
                                    96,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        2,
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
                                    45,
                                ),
                                opr: Comparison(
                                    Eq,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    95,
                                ),
                                ropd: SemaExprIdx(
                                    46,
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
                                            value: 530,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    99,
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
                                    48,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    100,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        14,
                                    ),
                                    variables: ArenaIdxRange(
                                        14..15,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 531,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    103,
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
                                    50,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    104,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        15,
                                    ),
                                    variables: ArenaIdxRange(
                                        15..16,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 531,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    109,
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
                                    52,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    110,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    53,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    111,
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
                                        112,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 53,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    114,
                                ),
                                current_symbol_idx: 16,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 16,
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
                                    117,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 123,
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
                                        4,
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
                                    116,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    56,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        4,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    55,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    115,
                                ),
                                ropd: SemaExprIdx(
                                    57,
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
                                            value: 530,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    121,
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
                                    59,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    122,
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
                                    60,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    123,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 418,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        124,
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
                                    125,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    126,
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
                            Literal(
                                RegionalTokenIdx(
                                    130,
                                ),
                                Bool(
                                    True,
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
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    61,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    127,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 358,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        128,
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
                                            parenate_parameters: [
                                                Regular(
                                                    FluffyTermRitchieRegularParameter {
                                                        contract: None,
                                                        ty: FluffyTerm {
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
                                                    },
                                                ),
                                            ],
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
                                    129,
                                ),
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: FluffyTerm {
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
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                62,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    131,
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
                                            value: 530,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    135,
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
                                    64,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    136,
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
                                    65,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    137,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 418,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        138,
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
                                    139,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    140,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    66,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    141,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 276,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        142,
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
                                            value: 53,
                                        },
                                    ),
                                    signature: PropsStruct {
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
                                            value: 530,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    146,
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
                                    68,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    147,
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
                                    69,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    148,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 418,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        149,
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
                                    150,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    151,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    70,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    152,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 277,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        153,
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
                                            value: 53,
                                        },
                                    ),
                                    signature: PropsStruct {
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
                                            value: 530,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    157,
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
                                    72,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    158,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    73,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    159,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 301,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        160,
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
                                                                value: 56,
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
                                                    value: 56,
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
                                    161,
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
                                        162,
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
                                    163,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    164,
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
                                            value: 530,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    168,
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
                                    76,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    169,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    77,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    170,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 301,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        171,
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
                                                                value: 56,
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
                                                    value: 56,
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
                                    172,
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
                                        173,
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
                                    174,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    175,
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
                                            value: 534,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    180,
                                ),
                                current_symbol_idx: 20,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 20,
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
                                            value: 535,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    182,
                                ),
                                current_symbol_idx: 21,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 21,
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
                                    80,
                                ),
                                opr: Closed(
                                    Add,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    181,
                                ),
                                ropd: SemaExprIdx(
                                    81,
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
                            Bracketed {
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    179,
                                ),
                                item: SemaExprIdx(
                                    82,
                                ),
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    183,
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
                                    185,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 124,
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
                                    83,
                                ),
                                opr: Closed(
                                    Div,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    184,
                                ),
                                ropd: SemaExprIdx(
                                    84,
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
                                            value: 531,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    189,
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
                                    86,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    190,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    87,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    191,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 301,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        192,
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
                                                                value: 56,
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
                                                    value: 56,
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
                                    88,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    193,
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
                                        194,
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
                                    195,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    196,
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
                                            value: 531,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    200,
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
                                    90,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    201,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    91,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    202,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 301,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        203,
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
                                                                value: 56,
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
                                                    value: 56,
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
                                    92,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    204,
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
                                        205,
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
                                    206,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    207,
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
                                            value: 537,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    212,
                                ),
                                current_symbol_idx: 23,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 23,
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
                                            value: 538,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    214,
                                ),
                                current_symbol_idx: 24,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 24,
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
                                    94,
                                ),
                                opr: Closed(
                                    Add,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    213,
                                ),
                                ropd: SemaExprIdx(
                                    95,
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
                            Bracketed {
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    211,
                                ),
                                item: SemaExprIdx(
                                    96,
                                ),
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    215,
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
                                    217,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 125,
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
                                    97,
                                ),
                                opr: Closed(
                                    Div,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    216,
                                ),
                                ropd: SemaExprIdx(
                                    98,
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
                                            value: 536,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    219,
                                ),
                                current_symbol_idx: 22,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 22,
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
                                            value: 539,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    221,
                                ),
                                current_symbol_idx: 25,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 25,
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
                                    100,
                                ),
                                opr: Comparison(
                                    Geq,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    220,
                                ),
                                ropd: SemaExprIdx(
                                    101,
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
                                            value: 529,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    223,
                                ),
                                current_symbol_idx: 7,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 7,
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
                                    225,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        3,
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
                                    103,
                                ),
                                opr: Comparison(
                                    Eq,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    224,
                                ),
                                ropd: SemaExprIdx(
                                    104,
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
                                            value: 530,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    228,
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
                                    106,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    229,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        26,
                                    ),
                                    variables: ArenaIdxRange(
                                        26..27,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 531,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    232,
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
                                    108,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    233,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        27,
                                    ),
                                    variables: ArenaIdxRange(
                                        27..28,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 532,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    236,
                                ),
                                current_symbol_idx: 12,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 12,
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
                                    110,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    237,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        28,
                                    ),
                                    variables: ArenaIdxRange(
                                        28..29,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 532,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    240,
                                ),
                                current_symbol_idx: 12,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 12,
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
                                    112,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    241,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    113,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    242,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 301,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        243,
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
                                                                value: 56,
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
                                                    value: 56,
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
                                    114,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    244,
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
                                        245,
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
                                    246,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    247,
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
                                    249,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 126,
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
                                    115,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    248,
                                ),
                                ropd: SemaExprIdx(
                                    116,
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
                                            value: 532,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    253,
                                ),
                                current_symbol_idx: 12,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 12,
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
                                    118,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    254,
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
                            Field {
                                owner_sema_expr_idx: SemaExprIdx(
                                    119,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    255,
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
                                        256,
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
                            PrincipalEntityPath {
                                path_expr_idx: 15,
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
                                                value: 42,
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
                                        20..38,
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
                                                value: 42,
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
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        98,
                                    ),
                                },
                                condition: SemaExprIdx(
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
                                        102,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    51,
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
                                        106,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        16,
                                    ),
                                    variables: ArenaIdxRange(
                                        16..17,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        108,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    54,
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
                                        113,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    58,
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
                                        118,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        17,
                                    ),
                                    variables: ArenaIdxRange(
                                        17..18,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        120,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    63,
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
                                        132,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        18,
                                    ),
                                    variables: ArenaIdxRange(
                                        18..19,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        134,
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
                                        143,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        19,
                                    ),
                                    variables: ArenaIdxRange(
                                        19..20,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        145,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    71,
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
                                        154,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        20,
                                    ),
                                    variables: ArenaIdxRange(
                                        20..21,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        156,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                                        165,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        21,
                                    ),
                                    variables: ArenaIdxRange(
                                        21..22,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        167,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    79,
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
                                        176,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        22,
                                    ),
                                    variables: ArenaIdxRange(
                                        22..23,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        178,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                                        186,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        23,
                                    ),
                                    variables: ArenaIdxRange(
                                        23..24,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        188,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                                        197,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        24,
                                    ),
                                    variables: ArenaIdxRange(
                                        24..25,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        199,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    93,
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
                                        208,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        25,
                                    ),
                                    variables: ArenaIdxRange(
                                        25..26,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        210,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    99,
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
                                        218,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    102,
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
                                        227,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    107,
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
                                        231,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    109,
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
                                        235,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    111,
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
                                        239,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    117,
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
                                        250,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        29,
                                    ),
                                    variables: ArenaIdxRange(
                                        29..30,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        252,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    120,
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
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    4,
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
                                        9,
                                    ),
                                },
                                condition: SemaExprIdx(
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
                                        13,
                                    ),
                                },
                                condition: SemaExprIdx(
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
                                        17,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    10,
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
                                        21,
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        7,
                                    ),
                                    variables: ArenaIdxRange(
                                        7..8,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        27,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    14,
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
                                        33,
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
                                        35,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    16,
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
                                        39,
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
                                        48,
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
                                        50,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    25,
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
                                        57,
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
                                        59,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    29,
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
                                        66,
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
                                        68,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    33,
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
                                        75,
                                    ),
                                },
                                condition: SemaExprIdx(
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        13,
                                    ),
                                    variables: ArenaIdxRange(
                                        13..14,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        81,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    41,
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
                                        89,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    44,
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
                                            93,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        47,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                97,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..15,
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
                                            222,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        105,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                226,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            15..20,
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
                                    121,
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
                                                value: 42,
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
                122,
                SemaExprIdx(
                    122,
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
                                        value: 31,
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
                                        value: 37,
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
                                        value: 33,
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 39,
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
                                        value: 29,
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
                                            value: 18,
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
                                            value: 46,
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
                                                value: 31,
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
                                                value: 37,
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
                                                value: 33,
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
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 39,
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
                                                value: 29,
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
                                                    value: 18,
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
                                                    value: 46,
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
            ],
        },
        sema_expr_terms: [
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
                    28,
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
                    32,
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
                    35,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    3,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    43,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        10.0,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    46,
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
                    56,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        180.0,
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
                    84,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        2.0,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    98,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        2.0,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    104,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    3,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    116,
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
                            EtherealTerm(`Leash OneVsAll MnistLabel Zero`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash OneVsAll MnistLabel Three`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash OneVsAll MnistLabel Seven`),
                        ),
                    },
                ),
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
                            EtherealTerm(`Leash OneVsAll MnistLabel Nine`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash OneVsAll MnistLabel Six`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`i32`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`EffHoles`),
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
                            EtherealTerm(`Option Leash ConcaveComponent`),
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
                            EtherealTerm(`Option Leash ConcaveComponent`),
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
                            EtherealTerm(`Option Leash ConcaveComponent`),
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
                                    19,
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
                                hole_source: Expr(
                                    28,
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
                                    32,
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
                                    55,
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
                    first_unresolved_term_idx: 5,
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
                                        EtherealTerm(`Leash OneVsAll MnistLabel Zero`),
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
                                        EtherealTerm(`Leash OneVsAll MnistLabel Three`),
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash OneVsAll MnistLabel Seven`),
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
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
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash OneVsAll MnistLabel Nine`),
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
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
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
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash OneVsAll MnistLabel Six`),
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
                                        EtherealTerm(`Leash Vec ConcaveComponent`),
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
                                        EtherealTerm(`i32`),
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
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 16,
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
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 17,
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
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 18,
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
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 19,
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
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 20,
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
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 20,
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
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 27,
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
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 28,
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
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 29,
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
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 29,
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
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 30,
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
                                idx: 34,
                                src: ExpectationSource {
                                    expr_idx: 31,
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
                                idx: 35,
                                src: ExpectationSource {
                                    expr_idx: 32,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            3,
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
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 33,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            3,
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
                                idx: 37,
                                src: ExpectationSource {
                                    expr_idx: 33,
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
                                idx: 38,
                                src: ExpectationSource {
                                    expr_idx: 34,
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
                                idx: 39,
                                src: ExpectationSource {
                                    expr_idx: 35,
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
                                idx: 40,
                                src: ExpectationSource {
                                    expr_idx: 36,
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
                                idx: 41,
                                src: ExpectationSource {
                                    expr_idx: 37,
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
                                idx: 42,
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
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 43,
                                src: ExpectationSource {
                                    expr_idx: 38,
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
                                idx: 44,
                                src: ExpectationSource {
                                    expr_idx: 40,
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
                                idx: 45,
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
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 46,
                                src: ExpectationSource {
                                    expr_idx: 42,
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
                                idx: 47,
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
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 48,
                                src: ExpectationSource {
                                    expr_idx: 44,
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
                                idx: 49,
                                src: ExpectationSource {
                                    expr_idx: 45,
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
                                idx: 50,
                                src: ExpectationSource {
                                    expr_idx: 46,
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
                                idx: 51,
                                src: ExpectationSource {
                                    expr_idx: 47,
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
                                idx: 52,
                                src: ExpectationSource {
                                    expr_idx: 48,
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
                                idx: 53,
                                src: ExpectationSource {
                                    expr_idx: 49,
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
                                idx: 54,
                                src: ExpectationSource {
                                    expr_idx: 50,
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
                                idx: 55,
                                src: ExpectationSource {
                                    expr_idx: 51,
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
                                idx: 56,
                                src: ExpectationSource {
                                    expr_idx: 52,
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
                                idx: 57,
                                src: ExpectationSource {
                                    expr_idx: 53,
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
                                idx: 58,
                                src: ExpectationSource {
                                    expr_idx: 54,
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
                                idx: 59,
                                src: ExpectationSource {
                                    expr_idx: 56,
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
                                idx: 60,
                                src: ExpectationSource {
                                    expr_idx: 55,
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
                                idx: 61,
                                src: ExpectationSource {
                                    expr_idx: 57,
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
                                idx: 62,
                                src: ExpectationSource {
                                    expr_idx: 58,
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
                                idx: 63,
                                src: ExpectationSource {
                                    expr_idx: 59,
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
                                idx: 64,
                                src: ExpectationSource {
                                    expr_idx: 60,
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
                                idx: 65,
                                src: ExpectationSource {
                                    expr_idx: 61,
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 66,
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
                                    expr_idx: 63,
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
                                idx: 68,
                                src: ExpectationSource {
                                    expr_idx: 64,
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
                                idx: 69,
                                src: ExpectationSource {
                                    expr_idx: 65,
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
                                idx: 70,
                                src: ExpectationSource {
                                    expr_idx: 66,
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
                                idx: 71,
                                src: ExpectationSource {
                                    expr_idx: 67,
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
                                idx: 72,
                                src: ExpectationSource {
                                    expr_idx: 68,
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
                                idx: 73,
                                src: ExpectationSource {
                                    expr_idx: 69,
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
                                idx: 74,
                                src: ExpectationSource {
                                    expr_idx: 70,
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
                                idx: 75,
                                src: ExpectationSource {
                                    expr_idx: 71,
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
                                idx: 76,
                                src: ExpectationSource {
                                    expr_idx: 72,
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
                                idx: 77,
                                src: ExpectationSource {
                                    expr_idx: 73,
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
                                idx: 78,
                                src: ExpectationSource {
                                    expr_idx: 74,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`RelativeBoundingBox`),
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
                                    expr_idx: 75,
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
                                idx: 80,
                                src: ExpectationSource {
                                    expr_idx: 76,
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
                                idx: 81,
                                src: ExpectationSource {
                                    expr_idx: 77,
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
                                idx: 82,
                                src: ExpectationSource {
                                    expr_idx: 78,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`RelativeBoundingBox`),
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
                                idx: 83,
                                src: ExpectationSource {
                                    expr_idx: 79,
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
                                    expr_idx: 80,
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
                                idx: 85,
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
                                idx: 86,
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
                                idx: 88,
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
                                idx: 89,
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
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 90,
                                src: ExpectationSource {
                                    expr_idx: 86,
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
                                idx: 91,
                                src: ExpectationSource {
                                    expr_idx: 87,
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
                                idx: 92,
                                src: ExpectationSource {
                                    expr_idx: 88,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`RelativeBoundingBox`),
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
                                idx: 93,
                                src: ExpectationSource {
                                    expr_idx: 89,
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
                                    expr_idx: 90,
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
                                idx: 95,
                                src: ExpectationSource {
                                    expr_idx: 91,
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
                                idx: 96,
                                src: ExpectationSource {
                                    expr_idx: 92,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`RelativeBoundingBox`),
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
                                idx: 97,
                                src: ExpectationSource {
                                    expr_idx: 93,
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
                                idx: 98,
                                src: ExpectationSource {
                                    expr_idx: 94,
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
                                idx: 99,
                                src: ExpectationSource {
                                    expr_idx: 95,
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
                                idx: 100,
                                src: ExpectationSource {
                                    expr_idx: 96,
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
                                idx: 101,
                                src: ExpectationSource {
                                    expr_idx: 97,
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
                                idx: 102,
                                src: ExpectationSource {
                                    expr_idx: 98,
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
                                idx: 103,
                                src: ExpectationSource {
                                    expr_idx: 99,
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
                                idx: 104,
                                src: ExpectationSource {
                                    expr_idx: 100,
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
                                idx: 105,
                                src: ExpectationSource {
                                    expr_idx: 101,
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
                                idx: 106,
                                src: ExpectationSource {
                                    expr_idx: 102,
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
                                idx: 107,
                                src: ExpectationSource {
                                    expr_idx: 103,
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
                                idx: 108,
                                src: ExpectationSource {
                                    expr_idx: 104,
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
                                idx: 109,
                                src: ExpectationSource {
                                    expr_idx: 105,
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
                                idx: 110,
                                src: ExpectationSource {
                                    expr_idx: 106,
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
                                idx: 111,
                                src: ExpectationSource {
                                    expr_idx: 107,
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
                                idx: 112,
                                src: ExpectationSource {
                                    expr_idx: 108,
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
                                idx: 113,
                                src: ExpectationSource {
                                    expr_idx: 109,
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
                                idx: 114,
                                src: ExpectationSource {
                                    expr_idx: 110,
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
                                idx: 115,
                                src: ExpectationSource {
                                    expr_idx: 111,
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
                                idx: 116,
                                src: ExpectationSource {
                                    expr_idx: 112,
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
                                idx: 117,
                                src: ExpectationSource {
                                    expr_idx: 113,
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
                                idx: 118,
                                src: ExpectationSource {
                                    expr_idx: 114,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`RelativeBoundingBox`),
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
                                idx: 119,
                                src: ExpectationSource {
                                    expr_idx: 115,
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
                                idx: 120,
                                src: ExpectationSource {
                                    expr_idx: 116,
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
                                idx: 121,
                                src: ExpectationSource {
                                    expr_idx: 117,
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
                                idx: 122,
                                src: ExpectationSource {
                                    expr_idx: 118,
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
                                idx: 123,
                                src: ExpectationSource {
                                    expr_idx: 119,
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
                                idx: 124,
                                src: ExpectationSource {
                                    expr_idx: 120,
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
                                            EtherealTerm(`OneVsAll MnistLabel Two`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 125,
                                src: ExpectationSource {
                                    expr_idx: 121,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Two`),
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
                                            EtherealTerm(`OneVsAll MnistLabel Two`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 126,
                                src: ExpectationSource {
                                    expr_idx: 122,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Two`),
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
            EtherealTerm(`OneVsAll MnistLabel Two`),
        ),
        self_ty: None,
    },
]