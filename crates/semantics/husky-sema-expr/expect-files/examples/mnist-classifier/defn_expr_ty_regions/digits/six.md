[
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                value: 37,
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
                            path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                                value: 37,
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
                                                value: 38,
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
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    9,
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
                                                5,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
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
                                    6,
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
                7,
                SemaExprIdx(
                    7,
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
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
                            path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                value: 34,
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
                                    3,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    9,
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
                                        10,
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
                                    12,
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
                                    4,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            5,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    13,
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
                                            value: 477,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    15,
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
                                    7,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    16,
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
                                path_expr_idx: 3,
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
                                    9,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    22,
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
                                        23,
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
                            PrincipalEntityPath {
                                path_expr_idx: 4,
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
                                    11,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    28,
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
                                path_expr_idx: 5,
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
                                    13,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    32,
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
                                        33,
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
                                    12,
                                ),
                                opr: Closed(
                                    Sub,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    30,
                                ),
                                ropd: SemaExprIdx(
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
                                            value: 259,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    35,
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
                                    16,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    36,
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
                                        37,
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
                                    39,
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
                                    17,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    38,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            18,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    40,
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
                                    19,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    41,
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
                                path_expr_idx: 6,
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
                            PrincipalEntityPath {
                                path_expr_idx: 7,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 34,
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
                                                value: 353,
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
                                path_expr_idx: 8,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 35,
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
                                    24,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    51,
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
                                        52,
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
                            Literal(
                                RegionalTokenIdx(
                                    56,
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
                                    21,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    45,
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
                                                    23,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        49,
                                                    ),
                                                ),
                                            },
                                            SemaVariadicCallListItem {
                                                argument_sema_expr_idx: SemaExprIdx(
                                                    25,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        53,
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
                                                54,
                                            ),
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 454,
                                                    },
                                                ),
                                            ),
                                            argument_sema_expr_idx: SemaExprIdx(
                                                26,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    57,
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
                                    27,
                                ),
                                opr: Unveil,
                                opr_regional_token_idx: RegionalTokenIdx(
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
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 9,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 35,
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
                                    29,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    63,
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
                                        64,
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
                                    66,
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
                                    30,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    65,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            31,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    67,
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
                                            value: 479,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    71,
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
                                    33,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    72,
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
                                    34,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    73,
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
                                        74,
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
                                    75,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
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
                                            value: 480,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    80,
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
                                    36,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    81,
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
                                        82,
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
                                            value: 477,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    86,
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
                                    38,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    87,
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
                                    39,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    88,
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
                                        89,
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
                                    90,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
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
                                    40,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    92,
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
                                        93,
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
                            PrincipalEntityPath {
                                path_expr_idx: 10,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 35,
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
                                    42,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    98,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 435,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        99,
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
                                                            value: 87,
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
                                                value: 87,
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
                                                value: 35,
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
                                    44,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    102,
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
                                        103,
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
                            Literal(
                                RegionalTokenIdx(
                                    105,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 63,
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
                                    45,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    104,
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
                                            value: 479,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    107,
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
                                    108,
                                ),
                                target: BePatternSynObelisk {
                                    pattern_expr: SynPatternRoot(
                                        12,
                                    ),
                                    variables: ArenaIdxRange(
                                        12..13,
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
                                            value: 481,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    112,
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
                            Literal(
                                RegionalTokenIdx(
                                    115,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 64,
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
                                        10,
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
                                    114,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    51,
                                ),
                            },
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
                            Binary {
                                lopd: SemaExprIdx(
                                    50,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    113,
                                ),
                                ropd: SemaExprIdx(
                                    52,
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
                                                value: 73,
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
                                    119,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 65,
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
                                    54,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    118,
                                ),
                                ropd: SemaExprIdx(
                                    55,
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
                                path_expr_idx: 13,
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
                                            value: 482,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    122,
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
                            Literal(
                                RegionalTokenIdx(
                                    126,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        15,
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
                                    57,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    121,
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
                                                    58,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        123,
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
                                                124,
                                            ),
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 454,
                                                    },
                                                ),
                                            ),
                                            argument_sema_expr_idx: SemaExprIdx(
                                                59,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    127,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        16,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Suffix {
                                opd_sema_expr_idx: SemaExprIdx(
                                    60,
                                ),
                                opr: Unveil,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    128,
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
                                                value: 28,
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
                                    63,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    137,
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
                                        138,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 477,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    142,
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
                                    65,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    143,
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
                                    66,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    144,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 151,
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
                                                                    value: 51,
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
                                                    value: 51,
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
                                    64,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    139,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 369,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        140,
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
                                                                                value: 51,
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
                                                                    value: 52,
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
                                    141,
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
                                                                    value: 51,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                67,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    148,
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
                                                    value: 52,
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
                                path_expr_idx: 17,
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
                                            value: 482,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    151,
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
                            PrincipalEntityPath {
                                path_expr_idx: 18,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 73,
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
                                    155,
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
                                path_expr_idx: 19,
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
                            Literal(
                                RegionalTokenIdx(
                                    161,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        6,
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
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    73,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    158,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 272,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        159,
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
                                                                                value: 18,
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
                                    160,
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
                                                                    value: 18,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                74,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    162,
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
                                    166,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        15,
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
                                    69,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    150,
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
                                                    70,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        152,
                                                    ),
                                                ),
                                            },
                                            SemaVariadicCallListItem {
                                                argument_sema_expr_idx: SemaExprIdx(
                                                    71,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        154,
                                                    ),
                                                ),
                                            },
                                            SemaVariadicCallListItem {
                                                argument_sema_expr_idx: SemaExprIdx(
                                                    72,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        156,
                                                    ),
                                                ),
                                            },
                                            SemaVariadicCallListItem {
                                                argument_sema_expr_idx: SemaExprIdx(
                                                    75,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        163,
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
                                                164,
                                            ),
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 454,
                                                    },
                                                ),
                                            ),
                                            argument_sema_expr_idx: SemaExprIdx(
                                                76,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    167,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        23,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Suffix {
                                opd_sema_expr_idx: SemaExprIdx(
                                    77,
                                ),
                                opr: Unveil,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    168,
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
                                            value: 483,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    170,
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
                                                    value: 52,
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
                                    79,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    171,
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
                                        172,
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
                                            value: 52,
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
                                    174,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 66,
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
                                    80,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    173,
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
                                path_expr_idx: 21,
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
                                                value: 28,
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
                                path_expr_idx: 23,
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
                                                value: 28,
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
                                path_expr_idx: 24,
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
                            PrincipalEntityPath {
                                path_expr_idx: 25,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 34,
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
                                    86,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    187,
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
                                        188,
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
                            Literal(
                                RegionalTokenIdx(
                                    192,
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
                                    85,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    185,
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
                                                    87,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        189,
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
                                                190,
                                            ),
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 454,
                                                    },
                                                ),
                                            ),
                                            argument_sema_expr_idx: SemaExprIdx(
                                                88,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    193,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        30,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Suffix {
                                opd_sema_expr_idx: SemaExprIdx(
                                    89,
                                ),
                                opr: Unveil,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    194,
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
                            PrincipalEntityPath {
                                path_expr_idx: 26,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 34,
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
                                    91,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    197,
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
                                        198,
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
                            Literal(
                                RegionalTokenIdx(
                                    200,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 67,
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
                                    92,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    199,
                                ),
                                ropd: SemaExprIdx(
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
                                path_expr_idx: 27,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 34,
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
                                    95,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    204,
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
                                        205,
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
                            Literal(
                                RegionalTokenIdx(
                                    207,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 68,
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
                                    96,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    206,
                                ),
                                ropd: SemaExprIdx(
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
                                            value: 259,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    209,
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
                                    99,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    210,
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
                                        211,
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
                                    213,
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
                                        32,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    100,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    212,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            101,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    214,
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
                            Suffix {
                                opd_sema_expr_idx: SemaExprIdx(
                                    102,
                                ),
                                opr: Unwrap,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    215,
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
                                    103,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    216,
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
                                        217,
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
                                    104,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    218,
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
                                        219,
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
                                    220,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    221,
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
                                    223,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 69,
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
                                    105,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    222,
                                ),
                                ropd: SemaExprIdx(
                                    106,
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
                                            value: 259,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    225,
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
                                    108,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    226,
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
                                        227,
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
                                    229,
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
                                        33,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    109,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    228,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            110,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    230,
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
                                    111,
                                ),
                                be_regional_token_idx: RegionalTokenIdx(
                                    231,
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
                                            value: 259,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    235,
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
                                    113,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    236,
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
                                        237,
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
                                    239,
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
                                        34,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    114,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    238,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            115,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    240,
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
                            Suffix {
                                opd_sema_expr_idx: SemaExprIdx(
                                    116,
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
                                    117,
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
                                    118,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    244,
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
                                                value: 70,
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
                                    119,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    248,
                                ),
                                ropd: SemaExprIdx(
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
                                            value: 478,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    251,
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
                                    253,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 71,
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
                                    122,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    252,
                                ),
                                ropd: SemaExprIdx(
                                    123,
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
                                path_expr_idx: 29,
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
                                                value: 28,
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
                                        20..32,
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
                                                value: 28,
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
                                        111,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    53,
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
                                        116,
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
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    61,
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
                            Return {
                                return_token: ReturnRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        129,
                                    ),
                                },
                                result: SemaExprIdx(
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
                            Return {
                                return_token: ReturnRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        176,
                                    ),
                                },
                                result: SemaExprIdx(
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
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    28,
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        59,
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
                                        61,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                                        68,
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
                                        70,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                                        77,
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
                                        79,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    37,
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
                                        83,
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
                                        85,
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        94,
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
                                        96,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                                        100,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    47,
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
                                            106,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        49,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                110,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..5,
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
                                        133,
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
                                        135,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    68,
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
                                    78,
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
                            IfElse {
                                sema_if_branch: SemaIfBranch {
                                    if_token: IfRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            169,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        82,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                175,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            5..6,
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
                            Return {
                                return_token: ReturnRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        180,
                                    ),
                                },
                                result: SemaExprIdx(
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
                                        202,
                                    ),
                                },
                                condition: SemaExprIdx(
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
                                        234,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    121,
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
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
                                        7,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                                        14,
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
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
                                        20,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
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
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        24,
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
                                        26,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    15,
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
                                            34,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        20,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                43,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            6..18,
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
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    90,
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
                            IfElse {
                                sema_if_branch: SemaIfBranch {
                                    if_token: IfRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            195,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        94,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                201,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            18..19,
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
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        208,
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
                            IfElse {
                                sema_if_branch: SemaIfBranch {
                                    if_token: IfRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            224,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        112,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                233,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            19..20,
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
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        250,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    124,
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
                                    125,
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
                                                value: 28,
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
                126,
                SemaExprIdx(
                    126,
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
                                        value: 87,
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
                                            value: 52,
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
                                                value: 87,
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
                                                    value: 52,
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
                    26,
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
                    31,
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
                    46,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        1.8,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    51,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        2.5,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    55,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        30.0,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    59,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    15,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    74,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    6,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    76,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    15,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    81,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        0.7,
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
                    93,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        1.8,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    97,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        1.8,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    101,
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
                    106,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        0.75,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    110,
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
                    115,
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
                    120,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        0.75,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    123,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        15.0,
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
                            EtherealTerm(`EffHoles`),
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
                            EtherealTerm(`Vector2d`),
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
                            EtherealTerm(`Vec Leash ConcaveComponent`),
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
                            EtherealTerm(`RelativePoint2d`),
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
                                    5,
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
                                    18,
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
                                    23,
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
                                    23,
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
                                                            value: 26,
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
                                EtherealTerm(`Six`),
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
                                    31,
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
                                    50,
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
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    62,
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
                                                11,
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
                                            12,
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
                                            11,
                                        ),
                                    ),
                                },
                                return_ty: FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            13,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    62,
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
                                                            value: 26,
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
                                EtherealTerm(`Six`),
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
                                                11,
                                            ),
                                        ),
                                    },
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                15,
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
                                            16,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    76,
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
                                                18,
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
                                            19,
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
                                            18,
                                        ),
                                    ),
                                },
                                return_ty: FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            20,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    76,
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
                                                            value: 26,
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
                                EtherealTerm(`Six`),
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
                                                18,
                                            ),
                                        ),
                                    },
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                22,
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
                                            23,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    94,
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
                                                25,
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
                                            26,
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
                                            25,
                                        ),
                                    ),
                                },
                                return_ty: FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            27,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    94,
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
                                                            value: 26,
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
                                EtherealTerm(`Six`),
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
                                                25,
                                            ),
                                        ),
                                    },
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                29,
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
                                            30,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    101,
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
                                    110,
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
                                    115,
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 7,
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
                                idx: 9,
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
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 9,
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
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 10,
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
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 11,
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
                                    expr_idx: 12,
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
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 14,
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
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 15,
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
                                idx: 17,
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
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 17,
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
                                    expr_idx: 18,
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
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 19,
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
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 19,
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
                                    expr_idx: 20,
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
                            expectation: Expectation::EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 21,
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
                                idx: 24,
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
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 23,
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
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 24,
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
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 25,
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
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 26,
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
                                            EtherealTerm(`OneVsAllResult MnistLabel Six`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 27,
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
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expectation(
                                        29,
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
                                                            value: 26,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expectation(
                                        29,
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
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 28,
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
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 29,
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
                                    expr_idx: 30,
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
                                    expr_idx: 31,
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
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 32,
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
                                idx: 37,
                                src: ExpectationSource {
                                    expr_idx: 32,
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
                                idx: 39,
                                src: ExpectationSource {
                                    expr_idx: 34,
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
                                idx: 40,
                                src: ExpectationSource {
                                    expr_idx: 35,
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
                                idx: 41,
                                src: ExpectationSource {
                                    expr_idx: 36,
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
                                idx: 42,
                                src: ExpectationSource {
                                    expr_idx: 37,
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
                                idx: 44,
                                src: ExpectationSource {
                                    expr_idx: 39,
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
                                idx: 45,
                                src: ExpectationSource {
                                    expr_idx: 40,
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
                                idx: 46,
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
                                idx: 47,
                                src: ExpectationSource {
                                    expr_idx: 42,
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
                                idx: 48,
                                src: ExpectationSource {
                                    expr_idx: 43,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Leash ConcaveComponent`),
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
                                idx: 49,
                                src: ExpectationSource {
                                    expr_idx: 44,
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
                                idx: 50,
                                src: ExpectationSource {
                                    expr_idx: 45,
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
                                idx: 51,
                                src: ExpectationSource {
                                    expr_idx: 46,
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
                                idx: 52,
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
                                idx: 53,
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
                                idx: 54,
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
                                idx: 55,
                                src: ExpectationSource {
                                    expr_idx: 51,
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
                                idx: 56,
                                src: ExpectationSource {
                                    expr_idx: 50,
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
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 57,
                                src: ExpectationSource {
                                    expr_idx: 52,
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
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 58,
                                src: ExpectationSource {
                                    expr_idx: 53,
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
                                idx: 59,
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
                                idx: 60,
                                src: ExpectationSource {
                                    expr_idx: 55,
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
                            expectation: Expectation::EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
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
                                                                    11,
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
                                                                    15,
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
                                                            16,
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
                                idx: 63,
                                src: ExpectationSource {
                                    expr_idx: 58,
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
                                idx: 64,
                                src: ExpectationSource {
                                    expr_idx: 59,
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
                                            EtherealTerm(`OneVsAllResult MnistLabel Six`),
                                        ),
                                    },
                                },
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
                                            16,
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
                                idx: 66,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expectation(
                                        65,
                                    ),
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
                                                            value: 26,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 67,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expectation(
                                        65,
                                    ),
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            15,
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
                                idx: 68,
                                src: ExpectationSource {
                                    expr_idx: 61,
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`OneVsAll MnistLabel Six`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 69,
                                src: ExpectationSource {
                                    expr_idx: 62,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Six`),
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
                                idx: 70,
                                src: ExpectationSource {
                                    expr_idx: 63,
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
                                idx: 71,
                                src: ExpectationSource {
                                    expr_idx: 64,
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
                                idx: 72,
                                src: ExpectationSource {
                                    expr_idx: 65,
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
                                    expr_idx: 66,
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                },
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
                                        EtherealTerm(`Point2d`),
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
                                idx: 75,
                                src: ExpectationSource {
                                    expr_idx: 68,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`RelativePoint2d`),
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
                                idx: 76,
                                src: ExpectationSource {
                                    expr_idx: 69,
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
                                                                    18,
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
                                                                    22,
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
                                                            23,
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
                                idx: 77,
                                src: ExpectationSource {
                                    expr_idx: 70,
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
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 78,
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
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 79,
                                src: ExpectationSource {
                                    expr_idx: 72,
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
                                idx: 80,
                                src: ExpectationSource {
                                    expr_idx: 73,
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
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 81,
                                src: ExpectationSource {
                                    expr_idx: 74,
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
                                idx: 83,
                                src: ExpectationSource {
                                    expr_idx: 76,
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
                                            EtherealTerm(`OneVsAllResult MnistLabel Six`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 84,
                                src: ExpectationSource {
                                    expr_idx: 77,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            23,
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
                                idx: 85,
                                src: ExpectationSource {
                                    expr_idx: 77,
                                    kind: Expectation(
                                        84,
                                    ),
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            18,
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
                                                            value: 26,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 86,
                                src: ExpectationSource {
                                    expr_idx: 77,
                                    kind: Expectation(
                                        84,
                                    ),
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            22,
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
                                idx: 87,
                                src: ExpectationSource {
                                    expr_idx: 78,
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
                                idx: 88,
                                src: ExpectationSource {
                                    expr_idx: 79,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`RelativePoint2d`),
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
                                idx: 89,
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
                                idx: 90,
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
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 91,
                                src: ExpectationSource {
                                    expr_idx: 82,
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
                                            EtherealTerm(`OneVsAll MnistLabel Six`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 92,
                                src: ExpectationSource {
                                    expr_idx: 83,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Six`),
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
                                            EtherealTerm(`OneVsAll MnistLabel Six`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 93,
                                src: ExpectationSource {
                                    expr_idx: 84,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Six`),
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
                            expectation: Expectation::EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 94,
                                src: ExpectationSource {
                                    expr_idx: 85,
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
                                                                    25,
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
                                                                    29,
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
                                                            30,
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
                                idx: 95,
                                src: ExpectationSource {
                                    expr_idx: 86,
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
                                idx: 96,
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
                                idx: 97,
                                src: ExpectationSource {
                                    expr_idx: 88,
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
                                            EtherealTerm(`OneVsAllResult MnistLabel Six`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 98,
                                src: ExpectationSource {
                                    expr_idx: 89,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            30,
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
                                idx: 99,
                                src: ExpectationSource {
                                    expr_idx: 89,
                                    kind: Expectation(
                                        98,
                                    ),
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            25,
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
                                                            value: 26,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 100,
                                src: ExpectationSource {
                                    expr_idx: 89,
                                    kind: Expectation(
                                        98,
                                    ),
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            29,
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
                                idx: 101,
                                src: ExpectationSource {
                                    expr_idx: 90,
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
                                idx: 102,
                                src: ExpectationSource {
                                    expr_idx: 91,
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
                                idx: 103,
                                src: ExpectationSource {
                                    expr_idx: 92,
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
                                idx: 104,
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
                                idx: 105,
                                src: ExpectationSource {
                                    expr_idx: 94,
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
                                idx: 106,
                                src: ExpectationSource {
                                    expr_idx: 95,
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
                                idx: 107,
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
                                idx: 108,
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
                                    expr_idx: 98,
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
                                    expr_idx: 99,
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
                                idx: 111,
                                src: ExpectationSource {
                                    expr_idx: 100,
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
                                idx: 112,
                                src: ExpectationSource {
                                    expr_idx: 101,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            32,
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
                                idx: 113,
                                src: ExpectationSource {
                                    expr_idx: 102,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            32,
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
                                idx: 114,
                                src: ExpectationSource {
                                    expr_idx: 102,
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
                                idx: 115,
                                src: ExpectationSource {
                                    expr_idx: 103,
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
                                idx: 116,
                                src: ExpectationSource {
                                    expr_idx: 104,
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
                                idx: 117,
                                src: ExpectationSource {
                                    expr_idx: 105,
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
                                idx: 118,
                                src: ExpectationSource {
                                    expr_idx: 106,
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
                                idx: 119,
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
                                idx: 120,
                                src: ExpectationSource {
                                    expr_idx: 108,
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
                                idx: 121,
                                src: ExpectationSource {
                                    expr_idx: 109,
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
                                idx: 122,
                                src: ExpectationSource {
                                    expr_idx: 110,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            33,
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
                                idx: 123,
                                src: ExpectationSource {
                                    expr_idx: 111,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            33,
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
                                idx: 124,
                                src: ExpectationSource {
                                    expr_idx: 111,
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
                                idx: 125,
                                src: ExpectationSource {
                                    expr_idx: 112,
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
                                idx: 126,
                                src: ExpectationSource {
                                    expr_idx: 113,
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
                                idx: 127,
                                src: ExpectationSource {
                                    expr_idx: 114,
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
                                idx: 128,
                                src: ExpectationSource {
                                    expr_idx: 115,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            34,
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
                                idx: 129,
                                src: ExpectationSource {
                                    expr_idx: 116,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            34,
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
                                idx: 130,
                                src: ExpectationSource {
                                    expr_idx: 116,
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
                                idx: 131,
                                src: ExpectationSource {
                                    expr_idx: 117,
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
                                idx: 132,
                                src: ExpectationSource {
                                    expr_idx: 118,
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
                                idx: 133,
                                src: ExpectationSource {
                                    expr_idx: 119,
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
                                idx: 134,
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
                                idx: 135,
                                src: ExpectationSource {
                                    expr_idx: 121,
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
                                idx: 136,
                                src: ExpectationSource {
                                    expr_idx: 122,
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
                                idx: 137,
                                src: ExpectationSource {
                                    expr_idx: 123,
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
                                idx: 138,
                                src: ExpectationSource {
                                    expr_idx: 124,
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
                                            EtherealTerm(`OneVsAll MnistLabel Six`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 139,
                                src: ExpectationSource {
                                    expr_idx: 125,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Six`),
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
                                            EtherealTerm(`OneVsAll MnistLabel Six`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 140,
                                src: ExpectationSource {
                                    expr_idx: 126,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Six`),
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
            EtherealTerm(`OneVsAll MnistLabel Six`),
        ),
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                                                value: 72,
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
                            path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                                    15,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 73,
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
                                    14,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 388,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    19,
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
                                    8,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    20,
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
                                        21,
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
                                            value: 388,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    23,
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
                                    10,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    24,
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
                                        25,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    9,
                                ),
                                opr: Closed(
                                    Div,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    22,
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
                            Bracketed {
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    18,
                                ),
                                item: SemaExprIdx(
                                    12,
                                ),
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    26,
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
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    13,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    27,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 61,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
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
                                    29,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
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
                                    32,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 74,
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
                                    14,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    31,
                                ),
                                ropd: SemaExprIdx(
                                    15,
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
                                    34,
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
                                    17,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    35,
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
                                        36,
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
                                    18,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    37,
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
                                        38,
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
                                    42,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 75,
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
                                    19,
                                ),
                                opr: Comparison(
                                    Less,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 287,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    46,
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
                                    22,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    47,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 220,
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
                                            value: 59,
                                        },
                                    ),
                                    signature: PropsStruct {
                                        ty: FluffyTerm {
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
                                    23,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    49,
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
                                        50,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 287,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    54,
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
                                    25,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    55,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 151,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        56,
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
                                                                    value: 51,
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
                                    57,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
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
                                                    value: 51,
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
                                    24,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    51,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 369,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        52,
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
                                                                                value: 51,
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
                                                                    value: 52,
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
                                    53,
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
                                                                    value: 51,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                26,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    59,
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
                                                    value: 52,
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
                                            value: 484,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    61,
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
                                                    value: 52,
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
                                    28,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    62,
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
                                        63,
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
                                            value: 52,
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
                                    65,
                                ),
                                Float(
                                    Unspecified(
                                        UnspecifiedFloatLiteral(
                                            Id {
                                                value: 76,
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
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    64,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 287,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    67,
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
                                    32,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    68,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 151,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        69,
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
                                                                    value: 51,
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
                                    70,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
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
                                                    value: 51,
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
                                    33,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    72,
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
                                        73,
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
                                            value: 51,
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
                            Prefix {
                                opr: Minus,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    66,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    34,
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
                                        2..8,
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
                                        17,
                                    ),
                                },
                                condition: SemaExprIdx(
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
                            IfElse {
                                sema_if_branch: SemaIfBranch {
                                    if_token: IfRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        7,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                16,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
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
                            Require {
                                require_token: RequireRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        33,
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
                                        45,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    27,
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
                                        60,
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
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    35,
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
                36,
                SemaExprIdx(
                    36,
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
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 52,
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
                                                    value: 52,
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
                    15,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                F32(
                                    NotNan(
                                        1.4,
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
                                F32(
                                    NotNan(
                                        0.6,
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
                                        0.5,
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
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`RelativePoint2d`),
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
                                idx: 9,
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 9,
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
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
                                resolve_progress: ExpectationProgress::Intact,
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
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 14,
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
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 15,
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
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 16,
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
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 17,
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
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 18,
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
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 19,
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
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 20,
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
                                idx: 21,
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
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 22,
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
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 23,
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
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 24,
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
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 25,
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 26,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
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
                                    expr_idx: 27,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`RelativePoint2d`),
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
                                    expr_idx: 28,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`RelativePoint2d`),
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
                                idx: 30,
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
                                idx: 31,
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 32,
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
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 33,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
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
                                    expr_idx: 34,
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
                                            EtherealTerm(`Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 35,
                                src: ExpectationSource {
                                    expr_idx: 35,
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
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 36,
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
]