[
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Prefix {
                                opr: Leash,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            ListFunctor {
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    13,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 2,
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Application {
                                function_sema_expr_idx: SemaExprIdx(
                                    3,
                                ),
                                argument_sema_expr_idx: SemaExprIdx(
                                    4,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
            (
                5,
                SemaExprIdx(
                    5,
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
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash ConnectedComponent`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vec Point2d`),
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
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
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
                                        EtherealTerm(`covariant Type -> Type`),
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
                                                        EtherealTerm(`Type`),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Covariant,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [],
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
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`r32`),
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`r32`),
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`r32`),
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 49,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
            (
                4,
                SemaExprIdx(
                    4,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`r32`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`r32`),
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 49,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 49,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Direction`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Direction`),
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 49,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 49,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
            (
                4,
                SemaExprIdx(
                    4,
                ),
            ),
            (
                5,
                SemaExprIdx(
                    5,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`r32`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`r32`),
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
                            EtherealTerm(`Direction`),
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
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
            current_symbol_map: [
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
                            EtherealTerm(`i32`),
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
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
                            ListFunctor {
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    6,
                                ),
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 2,
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
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Application {
                                function_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                                argument_sema_expr_idx: SemaExprIdx(
                                    2,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
            (
                4,
                SemaExprIdx(
                    4,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vec Point2d`),
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
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
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
                                        EtherealTerm(`covariant Type -> Type`),
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
                                                        EtherealTerm(`Type`),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Covariant,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Prefix {
                                opr: Leash,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            ListFunctor {
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 2,
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 48,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Application {
                                function_sema_expr_idx: SemaExprIdx(
                                    3,
                                ),
                                argument_sema_expr_idx: SemaExprIdx(
                                    4,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
            (
                5,
                SemaExprIdx(
                    5,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash ConnectedComponent`),
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
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
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
                                        EtherealTerm(`covariant Type -> Type`),
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
                                                        EtherealTerm(`Type`),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Covariant,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
        self_ty: None,
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
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
                                    Trait(
                                        TraitPath(
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
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 5,
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 48,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
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
                                        EtherealTerm(`Trait`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
            EtherealTerm(`RawContour`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    trai_path: TraitPath(`core::visual::Visualize`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `visualize`,
                                item_kind: MethodFn,
                            },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 39,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
            EtherealTerm(`RawContour`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 48,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
            EtherealTerm(`RawContour`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `line_segment_sketch`,
                                item_kind: MemoizedField,
                            },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 58,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
            EtherealTerm(`RawContour`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `bounding_box`,
                                item_kind: MemoizedField,
                            },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 55,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
            EtherealTerm(`RawContour`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `relative_bounding_box`,
                                item_kind: MemoizedField,
                            },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 56,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
            EtherealTerm(`RawContour`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `contour_len`,
                                item_kind: MemoizedField,
                            },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 28,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
            EtherealTerm(`RawContour`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::raw_contour`,
                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `displacement`,
                                item_kind: MethodFn,
                            },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
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
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
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
                            EtherealTerm(`i32`),
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
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
            EtherealTerm(`RawContour`),
        ),
    },
]